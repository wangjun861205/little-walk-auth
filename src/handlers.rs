use actix_web::{
    web::{Data, Query},
    HttpResponse,
};

use auth_service::core::{
    cacher::Cacher, hasher::Hasher, repository::Repository, secret_generator::SecretGenerator,
    service::{Service, VerifySecretRequest},
};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::VerificationCodeServiceAddress;

#[derive(Debug, Deserialize)]
pub struct LoginByVerificationCodeParams {
    phone: String,
    code: String,
}

#[derive(Debug, Serialize)]
pub struct LoginByVerificationResp {
    token: String,
}

pub async fn login_by_verification_code<R, C, H, S>(
    _service: Data<Service<R, C, H, S, String>>,
    Query(params): Query<LoginByVerificationCodeParams>,
    verification_code_service: Data<VerificationCodeServiceAddress>,
) -> HttpResponse
where
    R: Repository<String> + Clone,
    C: Cacher<String> + Clone,
    H: Hasher + Clone,
    S: SecretGenerator + Clone,
{
    match reqwest::Client::new()
        .get(format!(
            "http://{}/{}/{}",
            verification_code_service.0, params.phone, params.code
        ))
        .send()
        .await
    {
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Ok(res) => {
            let status = res.status();
            if status != StatusCode::OK {
                match res.text().await {
                    Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
                    Ok(text) => return HttpResponse::build(status).body(text),
                }
            }
            HttpResponse::Ok().json(LoginByVerificationResp {
                token: "test".into(),
            })
        }
    }
}


pub async fn verify_token<R, C, H, S>(
    service: Data<Service<R, C, H, S, String>>,
    Query(params): Query<LoginByVerificationCodeParams>,
    verification_code_service: Data<VerificationCodeServiceAddress>,
) -> HttpResponse
where
    R: Repository<String> + Clone,
    C: Cacher<String> + Clone,
    H: Hasher + Clone,
    S: SecretGenerator + Clone,
{
    service.verify_secret(VerifySecretRequest{})
}