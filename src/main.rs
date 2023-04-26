mod certificate;
mod storage;

use axum::{routing::{post, get}, Json, Router, http::StatusCode, response::IntoResponse,};
use axum::extract::Path;
use axum::extract::ConnectInfo;
use axum_macros::debug_handler;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
use std::env;

use serde::{Deserialize, Serialize};
use crate::certificate::certificate::gen_cert;
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Certificate {
    cert: String,
    key: String,
}

#[tokio::main]
async fn main() {
    let (_main_server, _rotate_server) = tokio::join!(start_main_server(), start_rotate_certs());
}

async fn start_rotate_certs() {
    loop {
        sleep(Duration::from_secs(36000)).await;
    }
}

async fn start_main_server() {
    let app = Router::new().route("/certificate", post(create_cert))
                           .route("/certificate/:name", get(get_cert))
                           .route("/auth", get(auth_cert));

    let addr = SocketAddr::from(([0, 0, 0, 0], 7104));
    //println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn generate_certificate() -> impl IntoResponse {
    let (cert, rsa) = gen_cert().unwrap();

    // Convert certificate and key to PEM format
    let cert_pem = cert.to_pem().unwrap();
    let key_pem = rsa.private_key_to_pem_pkcs8().unwrap();

    // Convert to base64-encoded JSON
    let cert_json = Certificate {
        cert: base64::encode(cert_pem),
        key: base64::encode(key_pem),
    };

    serde_json::to_string(&cert_json).unwrap()
}

async fn create_cert( Json(config): Json< String >) -> impl IntoResponse {
    return (StatusCode::BAD_REQUEST, axum::Json("{}".to_string()))
}

async fn get_cert(Path(name): Path<String>, Json(config): Json< String > ) -> impl IntoResponse {
    return (StatusCode::BAD_REQUEST, axum::Json("{}".to_string()))
}

async fn auth_cert( Json(certificate): Json< String >) -> impl IntoResponse {
    return (StatusCode::BAD_REQUEST, axum::Json("{}".to_string()))
}