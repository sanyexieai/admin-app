use crate::helpers::api_result::{APIDataResponse, APIResponse, APIerror};
use crate::helpers::db_helper;
use crate::models::permission_model::{
    CreatePermissionReq, EditPermissionReq, PermissionResponse
};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use dotenv::dotenv;
use entity::{permission, user};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::env;
use std::sync::Arc;


//插入权限
pub async fn create_permission(Json(permission_data): Json<CreatePermissionReq>) -> Result<Json<APIResponse>, APIerror> {
    dotenv().ok();
    let conn = db_helper::db_connection().await;
    let permission = permission::ActiveModel {
        name: Set(permission_data.name.to_owned()),
        r#type: Set(permission_data.r#type),
        sort: Set(permission_data.sort),
        is_admin_visible: Set(permission_data.is_admin_visible),
        is_user_visible: Set(permission_data.is_user_visible),
        description: Set(permission_data.description.to_owned()),
        parent_id: Set(permission_data.parent_id.to_owned()),
        icon: Set(permission_data.icon.to_owned()),
        key_name :Set(permission_data.key_name.into()),
        file_name:Set(permission_data.file_name.into()),
        path:Set(permission_data.path.into()),
        sofl_delete: Set(0),//0未删除
        status: Set(1),//1启用
        ..Default::default()
    };
    permission.save(&conn).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(APIResponse {
        message: "Create Success".to_string(),
        ..Default::default()
    }))
}

//更新权限
pub async fn edit_permission(Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>, Json(permission_data): Json<EditPermissionReq>) -> Result<Json<APIResponse>, APIerror> {
    dotenv().ok();
    let conn_db = db.as_ref();
    //根据id查询权限
    let mut permission: entity::permission::ActiveModel = permission::Entity::find()
    .filter(permission::Column::Id.eq(id))
    .one(conn_db)
    .await
    .map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ok_or_else(|| APIerror {
        message: "User not found".to_string(),
        status_code: StatusCode::NOT_FOUND,
    })?
    .into();
    //更新权限信息
    permission.name = Set(permission_data.name.to_owned());
    permission.r#type = Set(permission_data.r#type);
    permission.sort = Set(permission_data.sort);
    permission.is_admin_visible = Set(permission_data.is_admin_visible);
    permission.is_user_visible = Set(permission_data.is_user_visible);
    permission.description = Set(permission_data.description.to_owned());
    permission.parent_id = Set(permission_data.parent_id.to_owned());
    permission.icon = Set(permission_data.icon.to_owned());
    permission.key_name =Set(permission_data.key_name.into());
    permission.update_time = Set(Some(Utc::now().naive_local()));
    permission.file_name =Set(permission_data.file_name.into());
    permission.path =Set(permission_data.path.into());

    //更新权限
    permission.update(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(APIResponse {
        message: "Create Success".to_string(),
        ..Default::default()
    }))
}

//删除权限
pub async fn delete_permission(Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>) -> Result<Json<APIResponse>, APIerror> {
    dotenv().ok();
    let conn_db = db.as_ref();
    //根据id查询权限
    let mut permission: entity::permission::ActiveModel = permission::Entity::find()
    .filter(permission::Column::Id.eq(id))
    .one(conn_db)
    .await
    .map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ok_or_else(|| APIerror {
        message: "User not found".to_string(),
        status_code: StatusCode::NOT_FOUND,
    })?
    .into();
    //删除权限
    permission.sofl_delete = Set(1);
    permission.update(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(APIResponse {
        message: "Delete Success".to_string(),
        ..Default::default()
    }))
}

//查询权限列表
pub async fn get_permission_list(Extension(db): Extension<Arc<DatabaseConnection>>) -> Result<Json<APIDataResponse<Vec<PermissionResponse>>>, APIerror> {
    dotenv().ok();
    let conn_db = db.as_ref();
    //查询权限列表
    match permission::Entity::find().all(conn_db).await {
        Ok(permissions) => {
            let permission_list: Vec<PermissionResponse> = permissions
                .into_iter()
                .map(|permission| PermissionResponse {
                    id: permission.id,
                    name: permission.name,
                    r#type: permission.r#type,
                    sort: permission.sort,
                    is_admin_visible: permission.is_admin_visible,
                    is_user_visible: permission.is_user_visible,
                    description: permission.description,
                    parent_id: permission.parent_id,
                    icon: permission.icon,
                    key_name: permission.key_name,
                    create_time: permission.create_time,
                    update_time: permission.update_time,
                    sofl_delete: permission.sofl_delete,
                    status: permission.status,
                    path: permission.path,
                    create_user_id: permission.create_user_id,
                    update_user_id: permission.update_user_id,
                })
                .collect();
            Ok(Json(APIDataResponse {
                data: Some(permission_list),
                message: "Success".to_string(),
                ..Default::default()
            }))
        }
        Err(error) => return Err(APIerror {
            message: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}
