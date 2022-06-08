use actix_web::{web, HttpResponse, Responder};
use actix_web::http::Error;
use futures_util::StreamExt;
use sqlx::PgPool;
use web::BytesMut;


use crate::db::sample_broker::insert_user;
use crate::domain::sample_model::Sample;


#[tracing::instrument(
name = "Post a sample model",
skip(sample, pool),
fields(
id = % sample.id,
)
)]
pub async fn post_sample(sample: web::Json<Sample>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_user(sample.0, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
name = "Post a sample file to check the type",
skip(body, _pool),
)]
pub async fn post_sample_file(mut body: web::Payload, _pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    check_file(bytes.to_vec().as_slice()).await;
    //HttpResponse::Ok().finish()
    Ok(HttpResponse::Ok().finish())
}


pub async fn check_file(file_bytes: &[u8]) {
    let result = tree_magic::from_u8(file_bytes);
    println!("We got a file with a type of {}", result);
}


