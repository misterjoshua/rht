use clap::Parser;
use actix_web::{post,Responder,HttpResponse};
use log::{info};

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
    let url = url::Url::parse(url)?;

    if cfg!(target_os = "windows") {
        std::process::Command::new("start")
            .args([url.as_str()])
            .output()?;
    } else {
        std::process::Command::new("xdg-open")
            .args([url.as_str()])
            .output()?;
    }

    Ok(())
}

/// A web server that opens local web browsers.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ServerArgs {
    /// Listen on an address/port.
    #[arg(short, long, default_value_t = String::from("127.0.0.1:12345"))]
    listen: String
}

#[forbid(unsafe_code)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{HttpServer,App,middleware::Logger};

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let args = ServerArgs::parse();
    info!("Listening on http://{}", args.listen);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(open)
    })
        .bind(args.listen)?
        .run()
        .await?;

    Ok(())
}
