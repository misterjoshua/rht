#![windows_subsystem = "windows"]

use clap::Parser;
use actix_web::{post,Responder,HttpResponse};
use log::info;
use reqwest::StatusCode;
use url::Url;
use anyhow::anyhow;

/// A web server that opens local web browsers.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Open a url. When omitted, this tool runs a server.
    url: Option<String>,

    /// Listen on an address/port.
    #[arg(short, long, default_value_t = String::from("127.0.0.1:12345"))]
    listen: String
}

#[forbid(unsafe_code)]
#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let args = Args::parse();

    match args.url {
        Some(url) => client(&url, &args.listen).await,
        None => server(&args).await,
    }
}

async fn client(url: &str, listen: &str) -> Result<(), anyhow::Error> {
    use reqwest::Client;

    let url = Url::parse(url)?;
    info!("Opening url: {}", url);
    
    let server_url = Url::parse(&format!("http://{}/open", listen))?;
    
    let res = Client::new()
        .post(server_url)
        .body(url.to_string())
        .send()
        .await?;

    match res.status() {
        StatusCode::OK => Ok(()),
        _ => Err(anyhow!("Bad HTTP code: {}", res.status()))
    }
}

async fn server(args: &Args) -> Result<(), anyhow::Error> {
    use actix_web::{HttpServer,App,middleware::Logger};

    info!("Listening on http://{}", args.listen);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(open)
    })
        .bind(&args.listen)?
        .run()
        .await?;
    
    Ok(())
}

#[post("/open")]
async fn open(url: String) -> impl Responder {
    open_url(&url)
        .await
        .map(|_| {
            HttpResponse::Ok()
                .body(format!("Accepted url: {}\n", url))
        })
        .unwrap_or_else(|error| {
            HttpResponse::BadRequest()
                .body(format!("Error: {}\n", error))
        })
}

async fn open_url(url: &str) -> Result<(), anyhow::Error> {
    let url = Url::parse(url)?;

    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", url.as_str()])
            .output()?;
    } else {
        std::process::Command::new("xdg-open")
            .args([url.as_str()])
            .output()?;
    }

    Ok(())
}