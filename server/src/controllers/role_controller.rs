use crate::helpers::api_result::{APIDataResponse, APIResponse, APIerror};
use crate::helpers::db_helper;
use crate::models::role_model::{
    CreateRoleReq, RoleResponse
};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use dotenv::dotenv;
use entity::{role, role_permissions, user};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, Set, TransactionTrait
};
use std::env;
use std::sync::Arc;


pub async fn create_role(Json(role_data): Json<CreateRoleReq>) -> Result<Json<APIResponse>, APIerror> {
    let conn = db_helper::db_connection().await;

    let r#type = role_data.r#type.unwrap_or(1);
    let sort = role_data.sort.unwrap_or(1);

    // 开启事务
    let txn: DatabaseTransaction = conn.begin().await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let role = role::ActiveModel {
        name: Set(role_data.name.to_owned()),
        description: Set(role_data.description),
        r#type: Set(r#type),
        sort: Set(sort),
        sofl_delete: Set(0), // 0未删除
        status: Set(1), // 1启用
        ..Default::default()
    };

    // 插入角色
    let inserted_role = role.save(&txn).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    // 插入角色-权限关系
    for permission_id in role_data.permission_ids {
        let role_permission = role_permissions::ActiveModel {
            role_id: Set(inserted_role.id.clone().unwrap()),
            permission_id: Set(permission_id),
            sofl_delete: Set(0), // 0未删除
            ..Default::default()
        };

        role_permission.save(&txn).await.map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    }

    // 提交事务
    txn.commit().await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(APIResponse {
        message: "Create Success".to_string(),
        ..Default::default()
    }))
}


//查询所有角色
pub async fn get_all_role() -> Result<Json<APIDataResponse<Vec<RoleResponse>>>, APIerror> {
    dotenv().ok();
    let conn = db_helper::db_connection().await;

    match role::Entity::find().all(&conn).await {
        Ok(roles) => {
            let mut role_list: Vec<RoleResponse> = vec![];

            for role in roles {
                // 查询每个角色的权限ID
                let permissions = role_permissions::Entity::find()
                    .filter(role_permissions::Column::RoleId.eq(role.id))
                    .all(&conn)
                    .await;

                let permission_ids = match permissions {
                    Ok(permission_models) => permission_models
                        .into_iter()
                        .map(|perm| perm.permission_id)
                        .collect(),
                    Err(_) => vec![], // 处理错误，返回空的权限ID列表
                };

                role_list.push(RoleResponse {
                    id: role.id,
                    name: role.name,
                    r#type: role.r#type,
                    sort: role.sort,
                    description: role.description,
                    create_time: role.create_time,
                    update_time: role.update_time,
                    permission_ids:permission_ids,
                    icon: role.icon,
                });
            }

            Ok(Json(APIDataResponse {
                data: Some(role_list),
                message: "Success".to_string(),
                ..Default::default()
            }))
        }
        Err(error) => Err(APIerror {
            message: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}