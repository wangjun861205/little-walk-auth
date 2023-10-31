use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, Query},
    Error,
};

use auth_service::core::{
    hasher::Hasher, repository::Repository, service::Service, token_manager::TokenManager,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginByPasswordParams {
    phone: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginByPasswordResp {
    token: String,
}

pub async fn login_by_password<R, H, T>(
    service: Data<Service<R, H, T>>,
    Query(params): Query<LoginByPasswordParams>,
) -> Result<Json<LoginByPasswordResp>, Error>
where
    R: Repository + Clone,
    H: Hasher + Clone,
    T: TokenManager + Clone,
{
    Ok(Json(LoginByPasswordResp {
        token: service
            .login_by_password(&params.phone, &params.password)
            .await
            .map_err(|e| ErrorInternalServerError(e))?,
    }))
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenParams {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyTokenResp {
    id: String,
}

pub async fn verify_token<R, H, T>(
    service: Data<Service<R, H, T>>,
    Query(params): Query<VerifyTokenParams>,
) -> Result<Json<VerifyTokenResp>, Error>
where
    R: Repository + Clone,
    H: Hasher + Clone,
    T: TokenManager + Clone,
{
    let id = service
        .verify_token(&params.token)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;
    Ok(Json(VerifyTokenResp { id }))
}
