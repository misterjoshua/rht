use axum::Router;
use hyper::Body;
use std::net::SocketAddr;

pub struct ServeService<'a> {
    config: Vec<&'a dyn AxumConfig>,
    listener: SocketAddr,
}

impl<'a> ServeService<'_> {
    pub fn new(
        listener: &str,
        config: Vec<&'a dyn AxumConfig>,
    ) -> Result<ServeService<'a>, anyhow::Error> {
        Ok(ServeService {
            listener: listener.parse()?,
            config,
        })
    }

    pub async fn serve(&self) -> Result<(), anyhow::Error> {
        let app = self
            .config
            .iter()
            .fold(Router::new(), |app, config| config.config(app));

        axum::Server::bind(&self.listener)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

pub trait AxumConfig {
    fn config(&self, app: Router<Body>) -> Router<Body>;
}
