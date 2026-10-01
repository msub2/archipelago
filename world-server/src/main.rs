mod entity;
mod metrics;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;

use metrics::Metrics;

struct AppState {
    metrics: Metrics,
    registry: Registry,
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    state.metrics.homepage_requests.inc();

    HttpResponse::Ok().body("Hello, World!")
}

#[get("/metrics")]
async fn get_metrics(state: web::Data<AppState>) -> impl Responder {
    let mut buffer = String::new();

    encode(&mut buffer, &state.registry).expect("Failed to encode metrics!");

    HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut registry = Registry::default();

    let metrics = Metrics::new(&mut registry);

    let state = web::Data::new(AppState {
        metrics,
        registry,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(get_metrics)
        })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
