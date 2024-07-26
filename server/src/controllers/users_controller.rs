use crate::helpers::api_result::{APIDataResponse, APIResponse, APIerror};
use crate::helpers::db_helper;
use crate::middleware::auth::create_jwt;
use crate::models::permission_model::{PermissionResponse, PermissionVo};
use crate::models::users_model::{
    CreateUserReq, EditUserReq, LoginReq, LoginResponse, UserResponse, UserWithPermissionResponse
};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use entity::{permission, role, role_permissions, user, user_roles};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction,  EntityTrait, Iterable, JoinType, QueryFilter, QuerySelect, RelationTrait, Set, TransactionTrait
};

use std::collections::HashSet;
use std::sync::Arc;


#[debug_handler]
pub async fn create_user(Json(user_data): Json<CreateUserReq>) -> Result<Json<APIResponse>, APIerror> {
    let conn = db_helper::db_connection().await;
    let hashpassword = hash(&user_data.password, DEFAULT_COST).map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    //查询用户名是否存在
    let user_name = user::Entity::find()
        .filter(user::Column::UserName.eq(&user_data.username))
        .one(&conn)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    if user_name.is_some() {
        return Err(APIerror {
            message: "User name already exists".to_string(),
            status_code: StatusCode::CONFLICT,
        });
    }
    let user = user::ActiveModel {
        user_name: Set(user_data.username.to_owned()),
        password: Set(Some(hashpassword.to_owned())),
        update_time: Set(Some(Utc::now().naive_local())),
        create_time: Set(Some(Utc::now().naive_local())),
        sofl_delete: Set(0),//0未删除
        status: Set(1),//1启用
        sex:Set(Some(9)),// 9 未知
        ..Default::default()
    };

    println!("User data to insert: {:?}", user); // 打印调试信息

    let result = user.insert(&conn).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    });

    match result {
        Ok(_) => println!("Insert successful"), // 打印调试信息
        Err(err) => return Err(err), // 返回错误
    };

    Ok(Json(APIResponse {
        message: "Create Success".to_string(),
        ..Default::default()
    }))
}
#[debug_handler]
pub async fn login(
    Json(user_data): Json<LoginReq>,
) -> Result<Json<APIDataResponse<LoginResponse>>, APIerror> {
    let conn = db_helper::db_connection().await;
    match user::Entity::find()
        .filter(user::Column::UserName.eq(user_data.username))
        .one(&conn)
        .await
    {
        Ok(Some(user)) => {
            if let Some(ref hashed_password) = user.password {
                match verify(&user_data.password, &hashed_password) {
                    Ok(matches) if matches => {
                        match create_jwt(
                            &user.id.to_string(),
                            &user.user_name,
                        ).await {
                            Ok(token) => {
                                let token = LoginResponse {
                                    token,
                                };
                                let result = APIDataResponse{
                                    data: Some(token),
                                    message: "Login Success".to_string(),
                                    status_code: StatusCode::OK,
                                };
                                Ok(Json(result))
                            }
                            Err(_) => Err(APIerror {
                                message: "Login False ".to_string(),
                                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            }),
                        }
                    }
                    Ok(_) => Err(APIerror {
                        message: "Wrong password".to_string(),
                        status_code: StatusCode::NOT_FOUND,
                    }),
                    Err(_) => Err(APIerror {
                        message: "False to verity password".to_string(),
                        status_code: StatusCode::CONFLICT,
                    }),
                }
            } else {
                Err(APIerror {
                    message: "Wrong password".to_owned(),
                    status_code: StatusCode::NOT_FOUND,
                })
            }
        }

        Ok(None) => Err(APIerror {
            message: "user not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        }),
        Err(_) => Err(APIerror {
            message: "Database error".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}



#[debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
    Json(user_data): Json<EditUserReq>,
) -> Result<Json<APIResponse>, APIerror> {

    let conn = db_helper::db_connection().await;
    // 开启事务
    let txn: DatabaseTransaction = conn.begin().await.unwrap();

    // 哈希密码
    let hashpassword = hash(&user_data.new_password, DEFAULT_COST).unwrap();

    // 查找用户
    let user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Id.eq(id))
        .one(&txn)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    // 处理用户不存在的情况
    let user = user.ok_or_else(|| APIerror {
        message: "User not found".to_string(),
        status_code: StatusCode::NOT_FOUND,
    })?;

    // 创建一个 ActiveModel 来更新用户信息
    let mut user_active_model: user::ActiveModel = user.into();

    user_active_model.email = Set(user_data.new_email);
    user_active_model.user_name = Set(user_data.new_username);
    user_active_model.phone_number = Set(Some(user_data.new_phone));
    user_active_model.update_time = Set(Some(Utc::now().naive_local()));
    user_active_model.password = Set(Some(hashpassword));

    // 更新用户信息
    user_active_model.update(&txn).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    // 查询出所有旧的 userRoles 记录
    let old_roles= user_roles::Entity::find()
        .filter(user_roles::Column::UserId.eq(id))
        .all(&txn)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;
        // 逐个删除每条旧的 userRoles 记录
        for old_role in old_roles {
            let old_role_active_model: user_roles::ActiveModel = old_role.into();
            old_role_active_model.delete(&txn).await.map_err(|err| APIerror {
                message: err.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
        }
        match  user_data.role_list {
            Some(role_list) => {
                // 插入新的 userRoles 记录
                for role in role_list {
                    let new_user_role = user_roles::ActiveModel {
                        user_id: Set(id),
                        role_id: Set(role),
                        update_time: Set(Some(Utc::now().naive_local())),
                        create_time: Set(Some(Utc::now().naive_local())),
                        sofl_delete: Set(0),//0未删除
                        status: Set(1),//1启用
                        ..Default::default()
                    };

                    new_user_role.insert(&txn).await.map_err(|err| APIerror {
                        message: err.to_string(),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    })?;
                }
            },
            None => {},
        }

    // 提交事务
    txn.commit().await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(APIResponse {
        message: "Change Success".to_string(),
        ..Default::default()
    }))
}

#[debug_handler]
pub async fn delete_user(
    Path(id): Path<i32>,
) -> Result<Json<APIResponse>, APIerror> {
  let conn = db_helper::db_connection().await;
    let mut user: entity::user::ActiveModel = user::Entity::find()
        .filter(user::Column::Id.eq(id))
        .one(&conn)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or_else(|| APIerror {
            message: "User doesn't exists".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?
        .into();
    user.sofl_delete =Set(1);

    user.update(&conn).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(APIResponse {
        message: "Delete Success".to_string(),
        ..Default::default()
    }))
}


//获取当前登录用户
#[debug_handler]
pub async fn get_current_user(
    Extension(id): Extension<i32>,
) -> Result<Json<APIDataResponse<UserWithPermissionResponse>>, APIerror> {
    let conn = db_helper::db_connection().await;
    match user::Entity::find()
        .filter(user::Column::Id.eq(id))
        .one(&conn)
        .await
    {
        Ok(Some(user)) => {
            // 根据用户查询权限
            let permissions =  permission::Entity::find()
            .join_rev(
                JoinType::InnerJoin,
                role_permissions::Entity::belongs_to(permission::Entity)
                    .from(role_permissions::Column::PermissionId)
                    .to(permission::Column::Id)
                    .into()
            )
            .join(JoinType::InnerJoin, role_permissions::Relation::Role.def())
            .join_rev(
                JoinType::InnerJoin,
                user_roles::Entity::belongs_to(role::Entity)
                    .from(user_roles::Column::RoleId)
                    .to(role::Column::Id)
                    .into()
            )
            .join(JoinType::InnerJoin, user_roles::Relation::User.def())
            .select_only()
            .columns(permission::Column::iter())
            .filter(user::Column::Id.eq(user.id))
            .into_model::<PermissionVo>()  // 显式指定返回的模型
            .all(&conn)
            .await
            .map_err(|err| APIerror {
                message: err.to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
            let user_response = UserWithPermissionResponse {
                id: user.id,
                name:user.name,
                username: user.user_name,
                email: user.email,
                phonenumer: user.phone_number.map(|phonenumer| phonenumer.to_string()),
                create_time: user.create_time,
                update_time: user.update_time,
                sofl_delete: user.sofl_delete,
                status: user.status,
                permissions: Some(permissions),
            };

            Ok(Json(APIDataResponse {
                data: Some(user_response),
                message: "Success".to_string(),
                ..Default::default()
            }))
        }
        Ok(None) => Err(APIerror {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        }),
        Err(error) => return Err(APIerror {
            message: error.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
  }

#[debug_handler]
pub async fn get_all_user(
) -> Result<Json<APIDataResponse<Vec<UserResponse>>>, APIerror> {
    let conn = db_helper::db_connection().await;
    match user::Entity::find().all(&conn).await {
        Ok(users) => {
            let user_list: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: user.id,
                    username: user.user_name,
                    email: user.email,
                    phonenumer: user.phone_number.map(|phonenumer| phonenumer.to_string()),
                    create_time: user.create_time,
                    update_time: user.update_time,
                    sofl_delete: user.sofl_delete,
                    status: user.status,
                })
                .collect();
            Ok(Json(APIDataResponse {
                data: Some(user_list),
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
