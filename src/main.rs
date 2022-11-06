mod rht;

use std::{path::Path};

use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use clap::{Parser, Subcommand};
use log::info;

/// The default listener address
const LISTENER: &str = "127.0.0.1:12345";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Listen on an address/port.
    #[arg(short, long, default_value_t = String::from(LISTENER))]
    listener: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the server
    Serve,
    /// Open a URL
    Open(OpenArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct OpenArgs {
    /// The URL to open
    url: String,
}

#[forbid(unsafe_code)]
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let args = CliArgs::parse();
    let listener = args.listener;
    let command = args.command.unwrap_or(Commands::Serve);

    match command {
        Commands::Open(x) => {
            let local_path = Path::new(&x.url);
            let url = if local_path.is_file() {
                info!("Reading local file: {}", x.url);
                let bytes = std::fs::read(local_path)?;
                let contents = base64::encode(bytes);

                format!("data:application/octet-stream;base64,{}", base64::encode(contents))
            } else {
                x.url
            };

            let api_url = format!("http://{}/open", listener).parse()?;
            rht::json_api::JsonApi::new(api_url)
                .post(rht::open::OpenRequest::url(url.as_str()))
                .await?;
        }
        Commands::Serve => {
            axum::Server::bind(&listener.parse()?)
                .http1_max_buf_size(100*1024*1024)
                .http2_max_frame_size(100*1024*1024)
                .serve(Router::new()
                    .route("/open", post(open)).into_make_service())
                .await?;
        }
    }

    Ok(())
}

async fn open(Json(req): Json<rht::open::OpenRequest>) -> impl IntoResponse {
    match rht::open::open(&req).await {
        Ok(x) => (StatusCode::OK, Json(x)).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST).into_response(),
    }
}
