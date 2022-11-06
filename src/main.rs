mod rht;

use axum::{
    extract::{DefaultBodyLimit, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
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
            let req = rht::open::OpenRequest::from_user_input(x.url)?;
            let api_url = format!("http://{}/open", listener).parse()?;
            rht::json_api::JsonApi::new(api_url).post(req).await?;
        }
        Commands::Serve => {
            let app = Router::new()
                .route("/open", post(open))
                .layer(DefaultBodyLimit::disable());

            axum::Server::bind(&listener.parse()?)
                .serve(app.into_make_service())
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
