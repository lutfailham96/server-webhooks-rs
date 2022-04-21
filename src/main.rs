use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder, Result};
use clap::Parser;
use env_logger::Env;
use serde::Serialize;
use std::process::Command;

struct AppState {
    program_command: String,
}

#[derive(Parser)]
struct Args {
    #[clap(short = 'c', long, default_value = "true")]
    cmd: String,
    #[clap(short = 'p', long, default_value_t = 7000)]
    port: u16,
}

#[derive(Serialize)]
struct JsonResponse {
    status: String,
    data: String,
}

#[get("/")]
async fn root() -> Result<impl Responder> {
    let response = JsonResponse {
        status: "OK".to_string(),
        data: "Server running healthy".to_string(),
    };
    Ok(web::Json(response))
}

#[get("/webhooks")]
async fn webhook(data: web::Data<AppState>) -> Result<impl Responder> {
    let cmd = Command::new(&data.program_command)
        .output()
        .expect("Failed to run program");
    let response = JsonResponse {
        status: "OK".to_string(),
        data: String::from_utf8_lossy(&cmd.stdout).to_string(),
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let program_command = web::Data::new(AppState {
        program_command: args.cmd,
    });
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("Server running on http://0.0.0.0:{}", args.port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(program_command.clone())
            .service(root)
            .service(webhook)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
