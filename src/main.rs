#![windows_subsystem = "windows"]

mod rht;

use crate::rht::serve::AxumConfig;
use axum::{
    extract::{DefaultBodyLimit, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use clap::{Parser, Subcommand};
use hyper::Body;

/// The default listener address
const LISTENER: &str = "127.0.0.1:12345";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
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
    Open {
        /// The URL to open
        url: String,
    },
}

#[forbid(unsafe_code)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let args = Args::parse();

    let open = OpenService::create(&args.listener, "/open")?;

    let serve = rht::serve::ServeService::new(&args.listener, vec![&open])?;

    match args.command.unwrap_or(Commands::Serve) {
        Commands::Open { url } => open.send(url).await,
        Commands::Serve => serve.serve().await,
    }
}

struct OpenService<'a> {
    api: rht::json_api::JsonApi<rht::open::OpenRequest, rht::open::OpenResponse>,
    path: &'a str,
}

impl<'a> OpenService<'_> {
    pub fn create(listener: &'a str, path: &'a str) -> Result<OpenService<'a>, anyhow::Error> {
        let url = format!("http://{}{}", listener, path).parse()?;

        Ok(OpenService {
            api: rht::json_api::JsonApi::new(url),
            path,
        })
    }

    pub async fn send(&self, url: String) -> anyhow::Result<()> {
        let req = rht::open::OpenRequest::from_user_input(url)?;
        self.api.post(req).await?;
        Ok(())
    }
}

impl AxumConfig for OpenService<'_> {
    fn config(&self, app: Router<Body>) -> Router<Body> {
        async fn open_service(Json(req): Json<rht::open::OpenRequest>) -> impl IntoResponse {
            match rht::open::open(&req).await {
                Ok(x) => (StatusCode::OK, Json(x)).into_response(),
                Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
            }
        }

        app.route(self.path, post(open_service))
            .layer(DefaultBodyLimit::disable())
    }
}
