mod config;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use config::AppConfig;

use std::io::stdout;
use std::str::FromStr;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

use tracing::error;

fn setup_tracing(log_level: &str) {
    let level = LevelFilter::from_str(log_level).unwrap_or(LevelFilter::TRACE);

    tracing_subscriber::fmt()
        .json()
        .with_max_level(level)
        .with_thread_names(true)
        .with_current_span(false)
        .flatten_event(true)
        .with_span_events(FmtSpan::NONE)
        .with_writer(stdout)
        .init();

    std::panic::set_hook(Box::new(|panic| {
        if let Some(location) = panic.location() {
            error!(
                message = %panic,
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column(),
            )
        } else {
            error!(message = %panic);
        }
    }));
}

#[actix_web::main]
async fn main() {
    let config = AppConfig::new();

    setup_tracing(&config.logging.level);

    HttpServer::new(move || App::new().wrap(TracingLogger::default()).service(health))
        .bind((config.server.host, config.server.port))
        .expect("Unable to bind server")
        .run()
        .await
        .expect("Failed to start web server")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
