use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct APIDataResponse<T> {
    pub data: Option<T>,
    pub message: String,
    #[serde(serialize_with = "serialize_status_code", deserialize_with = "deserialize_status_code")]
    pub status_code: StatusCode,
}
// 实现 Default trait
impl<T> Default for APIDataResponse<T> {
    fn default() -> Self {
        APIDataResponse {
            data: None,
            message: "".to_string(),
            status_code: StatusCode::OK,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct APIResponse {
    pub message: String,
    #[serde(serialize_with = "serialize_status_code", deserialize_with = "deserialize_status_code")]
    pub status_code: StatusCode,
}

// 实现 Default trait
impl Default for APIResponse {
    fn default() -> Self {
        APIResponse {
            message: "".to_string(),
            status_code: StatusCode::OK,
        }
    }
}

// 自定义 StatusCode 的序列化方法
fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

// 自定义 StatusCode 的反序列化方法
fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let code = u16::deserialize(deserializer)?;
    StatusCode::from_u16(code).map_err(serde::de::Error::custom)
}

#[derive(Debug)]
pub struct APIerror {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for APIerror {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        (
            status_code,
            [(header::CONTENT_TYPE, "application/json")],
            Json(json!({
                "StatusCode": self.status_code.as_u16(),
                "Message": self.message
            })),
        )
            .into_response()
    }
}
