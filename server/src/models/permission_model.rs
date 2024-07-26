use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct CreatePermissionReq{
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub icon: Option<String>,
    pub sort: i32,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub create_user_id: Option<i32>,
    pub update_user_id: Option<i32>,
    pub is_admin_visible: i32,
    pub is_user_visible: i32,
    pub key_name: String,
    pub file_name: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct EditPermissionReq{
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub icon: Option<String>,
    pub sort: i32,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub create_user_id: Option<i32>,
    pub update_user_id: Option<i32>,
    pub is_admin_visible: i32,
    pub is_user_visible: i32,
    pub key_name: String,
    pub file_name: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct PermissionResponse{
    pub id:i32,
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub icon: Option<String>,
    pub sort: i32,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time:  Option<NaiveDateTime>,
    pub create_user_id: Option<i32>,
    pub update_user_id: Option<i32>,
    pub is_admin_visible: i32,
    pub is_user_visible: i32,
    pub key_name: Option<String>,
    pub sofl_delete: i32,
    pub status: i32,
}


#[derive(Serialize,Deserialize,FromQueryResult)]
pub struct PermissionVo{
    pub id:i32,
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub sort: i32,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub key_name: Option<String>,
    pub status: i32,
}
