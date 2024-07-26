use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Debug)]
pub struct CreateRoleReq{
    pub name: String,
    pub description: String,
    pub r#type: Option<i32>,
    pub icon: Option<String>,
    pub sort:  Option<i32>,
    pub permission_ids: Vec<i32>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct RoleResponse{
    pub id: i32,
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub icon: Option<String>,
    pub sort: i32,
    pub permission_ids: Vec<i32>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time:  Option<NaiveDateTime>,
}