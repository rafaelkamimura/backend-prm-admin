use crate::application::services::CharacterService;
use actix_web::{web, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/characters",
    responses(
        (status = 200, description = "Get all characters", body = [Character])
    )
)]
pub async fn get_characters<R>(service: web::Data<CharacterService<R>>) -> impl Responder
where
    R: crate::domain::repositories::character_repository::CharacterRepository
        + Send
        + Sync
        + 'static,
{
    match service.get_all_characters().await {
        Ok(characters) => HttpResponse::Ok().json(characters),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    get,
    path = "/characters/online",
    responses(
        (status = 200, description = "Get online characters", body = [Character])
    )
)]
pub async fn get_online_characters<R>(service: web::Data<CharacterService<R>>) -> impl Responder
where
    R: crate::domain::repositories::character_repository::CharacterRepository
        + Send
        + Sync
        + 'static,
{
    match service.get_online_characters().await {
        Ok(characters) => HttpResponse::Ok().json(characters),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
