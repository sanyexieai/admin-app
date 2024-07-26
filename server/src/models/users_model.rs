use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::permission_model::{PermissionVo};


#[derive(Deserialize,Serialize,Debug)]
pub struct Users{
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub password: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct CreateUserReq{
    pub username:String,
    pub password:String,

}
#[derive(Serialize,Deserialize)]
pub struct LoginReq{
    pub username:String,
    pub password:String,
    //角色列表
    pub role_list:Option<Vec<i32>>,

}

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
    pub token: String,
}

#[derive(Serialize,Deserialize)]
pub struct EditUserReq{
    pub new_username: String,
    pub new_email: Option<String>,
    pub new_phone: String,
    pub new_password: String,
    //角色列表
    pub role_list:Option<Vec<i32>>,
}

#[derive(Serialize,Deserialize)]
pub struct UserWithPermissionResponse{
    pub id:i32,
    pub username: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phonenumer: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub status: i32,
    pub sofl_delete: i32,
    pub permissions: Option<Vec<PermissionVo>>,
}

#[derive(Serialize,Deserialize)]
pub struct UserResponse{
    pub id:i32,
    pub username: String,
    pub email: Option<String>,
    pub phonenumer: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: Option<NaiveDateTime>,
    pub status: i32,
    pub sofl_delete: i32,
}
