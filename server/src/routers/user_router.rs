use axum::{
    middleware, routing::{get, post}, Extension, Router
};
use sea_orm::DatabaseConnection;
use crate::{controllers::users_controller::{
    create_user, delete_user, update_user, get_all_user, get_current_user, login
}, middleware::auth::guards};
use std::sync::Arc;


//add extension to match with controller can check sync connect database
pub fn user_router(db : Extension<Arc<DatabaseConnection>>) -> Router {
    //开放接口
    let pub_router = Router::new()
        .route("/api/v1/users/create_user", post(create_user))
        .route("/api/v1/users/login", post(login));
    //私有接口
    let priv_router = Router::new()
        .route("/api/v1/users/info", get(get_current_user))
        .route("/api/v1/users/update_user/:id", post(update_user))
        .route("/api/v1/users/delete_user/:id", post(delete_user))
        .route("/api/v1/users/get_all_user", get(get_all_user))
        .layer(middleware::from_fn(guards));
    Router::new().merge(pub_router).merge(priv_router)
}
