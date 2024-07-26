use axum::{
    middleware, routing::{get, post}, Extension, Router
};
use sea_orm::DatabaseConnection;
use crate::{controllers::permission_controller::{create_permission, delete_permission, edit_permission, get_permission_list
}, middleware::auth::guards};
use std::sync::Arc;


//add extension to match with controller can check sync connect database
pub fn permission_router(db : Extension<Arc<DatabaseConnection>>) -> Router {
    // //角色相关接口
    // let role_router = Router::new()
    //     .route("/api/v1/create_role", post(create_role))
    //     .route("/api/v1/update_role/:id", post(edit_role))
    //     .route("/api/v1/delete_role/:id", post(delete_role))
    //     .route("/api/v1/get_all_role", get(get_all_role));
    // Router::new().merge(permission_router);
    let priv_router = Router::new()
    .route("/api/v1/permissions/create_permission", post(create_permission))
    .route("/api/v1/permissions/update_permission/:id", post(edit_permission))
    .route("/api/v1/permissions/delete_permission/:id", post(delete_permission))
    .route("/api/v1/permissions/get_all_permission", get(get_permission_list))
    .layer(middleware::from_fn(guards));
    Router::new().merge(priv_router)
}
