use actix_web::{web, Scope};
pub(crate) mod authentication_middleware;
pub mod controller;
pub mod dtos;
mod error;
mod service;

pub(crate) fn auth_module() -> Scope {
    web::scope("/auth").route(
        "/create-session",
        web::post().to(controller::create_session),
    )
}
