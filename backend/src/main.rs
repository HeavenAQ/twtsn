use std::{fs::File, io::Read, net::TcpListener};

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Port 8080 is already in use");
    let _ = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ]);
        App::new().configure(config).wrap(cors)
    })
    .listen(listener)?
    .run()
    .await;
    Ok(())
}

pub fn respond_failure(message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "status": "failure",
        "message": message
    }))
}

fn parse_headlines(data: String) -> Vec<(String, String)> {
    let headline_info = data.lines().map(|line| {
        let (headline, url) = line.split_once("---").unwrap();
        (headline.trim().to_string(), url.trim().to_string())
    });
    headline_info.collect()
}

#[actix_web::get("/headlines")]
pub async fn get_headlines_handler() -> impl Responder {
    let mut data = String::new();
    let mut file = match File::open("../../modules/news/.news_headlines") {
        Ok(file) => file,
        Err(e) => return respond_failure(&e.to_string()),
    };

    match file.read_to_string(&mut data) {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "headlines": parse_headlines(data)
        })),
        Err(e) => respond_failure(&e.to_string()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("api").service(get_headlines_handler);
    conf.service(scope);
}
