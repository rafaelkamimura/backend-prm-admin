mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::application::services::CharacterService;
use crate::infrastructure::persistence::character_repository::MySqlCharacterRepository;
use crate::presentation::handlers::character_handlers::{
    __path_get_characters, __path_get_online_characters,
};
use crate::presentation::handlers::character_handlers::{get_characters, get_online_characters};
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(get_characters, get_online_characters))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let character_repository = MySqlCharacterRepository::new(pool.clone());
    let character_service = CharacterService::new(character_repository);

    HttpServer::new(move || {
        let api = App::new()
            .app_data(web::Data::new(character_service.clone()))
            .service(
                web::resource("/characters")
                    .route(web::get().to(get_characters::<MySqlCharacterRepository>)),
            )
            .service(
                web::resource("/characters/online")
                    .route(web::get().to(get_online_characters::<MySqlCharacterRepository>)),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .route(
                "/api-doc/openapi.json",
                web::get().to(|| async { HttpResponse::Ok().json(ApiDoc::openapi()) }),
            );

        api
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
