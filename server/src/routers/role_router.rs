use axum::{
    middleware, routing::{get, post}, Extension, Router
};
use sea_orm::DatabaseConnection;
use crate::{controllers::role_controller::{create_role, get_all_role
}, middleware::auth::guards};
use std::sync::Arc;


//add extension to match with controller can check sync connect database
pub fn role_router(db : Extension<Arc<DatabaseConnection>>) -> Router {
    // //角色相关接口
    // let role_router = Router::new()
    //     .route("/api/v1/create_role", post(create_role))
    //     .route("/api/v1/update_role/:id", post(edit_role))
    //     .route("/api/v1/delete_role/:id", post(delete_role))
    //     .route("/api/v1/get_all_role", get(get_all_role));
    // Router::new().merge(role_router);
    let priv_router = Router::new()
        .route("/api/v1/roles/create_role", post(create_role))
        //     .route("/api/v1/update_role/:id", post(edit_role))
        //     .route("/api/v1/delete_role/:id", post(delete_role))
        .route("/api/v1/roles/get_all_role", get(get_all_role))
        .layer(middleware::from_fn(guards));
    Router::new().merge(priv_router)
}
