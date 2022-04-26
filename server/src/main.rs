use std::sync::RwLock;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use adorable::FaceFactory;

pub async fn create_avatar(
    face_factory: web::Data<RwLock<FaceFactory>>,
    size: u32,
    value: String,
) -> impl Responder {
    let image_data = web::block(move || {
        log::info!("Request {:?}", &value);
        let image_data = adorable::create_avatar(face_factory.read().unwrap(), size, &value);
        image_data
    })
    .await
    .unwrap_or(vec![]);

    HttpResponse::Ok()
        .content_type("image/JPEG")
        .body(web::Bytes::from(image_data))
}

#[get("/{size}/{random}")]
async fn avatar_with_custom_value(
    face_factory: web::Data<RwLock<FaceFactory>>,
    path: web::Path<(u32, String)>,
) -> impl Responder {
    let (size, value) = path.into_inner();
    create_avatar(face_factory, size, value).await
}

#[get("/{size}/random")]
async fn avatar_with_random_value(
    face_factory: web::Data<RwLock<FaceFactory>>,
    path: web::Path<u32>,
) -> impl Responder {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    create_avatar(face_factory, *path, rand_string).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let face_factory = adorable::create_face_factory();
    let face_factory = web::Data::new(RwLock::new(face_factory));

    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    log::warn!("Server started on port {:?}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(face_factory.clone())
            .service(avatar_with_random_value)
            .service(avatar_with_custom_value)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
