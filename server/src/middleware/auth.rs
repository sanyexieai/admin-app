use crate::helpers::{self, api_result::APIerror, db_helper};
use ::entity::user::Entity;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use entity::user::Column;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait,EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use helpers::dev_helper::get_dev;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    sub: String,
    pub username: String,
}

pub async fn create_jwt(id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(2))
        .expect("valid  timestamp")
        .timestamp();
    let my_secret_key: String = get_dev("MY_SECRET_KEY".to_string(), Some("sanyexieai".to_string())).await;
    let claims = Claims {
        sub: id.to_owned(),
        exp: exp as usize,
        username: username.to_owned(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(my_secret_key.as_ref()),
    )?;
    Ok(token)
}

async fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let my_secret_key: String =  get_dev("MY_SECRET_KEY".to_string(), Some("sanyexieai".to_string())).await;
    let validation = Validation::default();
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(my_secret_key.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
pub async fn guards(req: Request, next: Next) -> Result<Response, APIerror> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| {
            let error_message = "No Auth token found".to_owned();
            println!("Error: {}", error_message);
            APIerror {
                message: error_message,
                status_code: StatusCode::BAD_REQUEST,
            }
        })?
        .to_str()
        .map_err(|err| {
            let error_message = format!("Failed to convert token to string. Error: {:?}", err);
            println!("{}", error_message);
            APIerror {
                message: error_message,
                status_code: StatusCode::BAD_REQUEST,
            }
        })?
        .trim();

    if !token.starts_with("Bearer ") {
        let error_message = "Authorization header must start with Bearer".to_owned();
        println!("Error: {}", error_message);
        return Err(APIerror {
            message: error_message,
            status_code: StatusCode::BAD_REQUEST,
        });
    }
    let token = &token[7..];
    let claim = verify_token(token).await.map_err(|err| {
        println!("Error verifying JWT: {:?}", err);
        APIerror {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
        }
    })?;

    let conn = db_helper::db_connection().await;
    let identity = Entity::find()
        .filter(Column::UserName.eq(claim.username.to_lowercase()))
        .one(&conn)
        .await
        .unwrap();

    let mut  req = req;
    if let Some(identity) = identity {
      // 将 `identity` 信息附加到请求的扩展中
      req.extensions_mut().insert(identity.id);
  }
    Ok(next.run(req).await)
}
