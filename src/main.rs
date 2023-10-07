use actix_cors::Cors;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{
    get,
    http::header::{self, LOCATION},
    post,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use num_cpus;
use std::{env, fs::File, io::Write};

#[get("/")]
async fn index() -> HttpResponse {
    dotenv().ok();
    let location = env::var("REDIRECT_URI").unwrap();
    HttpResponse::Found()
        .append_header((LOCATION, location))
        .finish()
}

#[get("/delete/{filename}")]
async fn delete(filename: web::Path<String>, req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let auth = match headers.get("Authorization") {
        Some(auth) => auth.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    dotenv().ok();
    let pw = env::var("PASSWORD").unwrap();

    let filepath = format!("./i/{}", filename);
    if auth == pw {
        match std::fs::remove_file(filepath) {
            Ok(_) => HttpResponse::Ok().body("ok"),
            Err(_) => HttpResponse::InternalServerError().body("error"),
        }
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[post("/upload")]
async fn image_upload(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    let headers = req.headers();
    let auth = match headers.get("Authorization") {
        Some(auth) => auth.to_str().unwrap().to_string(),
        None => return HttpResponse::Unauthorized().body("Unauthorized"),
    };

    dotenv().ok();
    let pw = env::var("PASSWORD").unwrap();

    if auth == pw {
        while let Ok(Some(mut field)) = payload.try_next().await {
            let content_type = field.content_disposition();
            let filename = content_type.get_filename().unwrap().to_string();
            let extension = filename
                .split('.')
                .last()
                .unwrap()
                .to_string()
                .to_lowercase();

            if !["jpg", "jpeg", "png", "gif", "webp"].contains(&extension.as_str()) {
                return HttpResponse::BadRequest().body("invalid file type");
            }

            let mut f = web::BytesMut::new();
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(chunk) => chunk,
                    Err(_) => {
                        return HttpResponse::InternalServerError().body("incompleted upload");
                    }
                };
                f.extend_from_slice(&data);
            }

            let filepath = format!("./i/{}", filename);

            let mut file = File::create(filepath).unwrap();
            file.write_all(&f).unwrap();
        }
        HttpResponse::Ok().body("ok")
    } else {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num_workers = num_cpus::get();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allow_any_header(),
            )
            .service(Files::new("/i/", "./i/"))
            .service(image_upload)
            .service(delete)
            .service(index)
    })
    .workers(num_workers)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
