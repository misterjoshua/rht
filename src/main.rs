mod rhcli;

use axum::{extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router};
use clap::{Parser, Subcommand};

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
            let api_url = format!("http://{}/open", listener).parse()?;
            rhcli::json_api::JsonApi::new(api_url)
                .post(rhcli::open::OpenRequest::new(x.url.as_str()))
                .await?;
        }
        Commands::Serve => {
            axum::Server::bind(&listener.parse()?)
                .serve(Router::new().route("/open", post(open)).into_make_service())
                .await?;
        }
    }

    Ok(())
}

async fn open(Json(req): Json<rhcli::open::OpenRequest>) -> impl IntoResponse {
    match rhcli::open::open(&req).await {
        Ok(x) => (StatusCode::OK, Json(x)).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST).into_response(),
    }
}
