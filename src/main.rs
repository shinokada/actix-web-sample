use actix_web::{get, middleware, App, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

// todo moduleをインポート
mod todos;

#[get("/hello")]
async fn hello_world() -> impl Responder {
    format!("Hello World!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .configure(todos::init_routes)
            .wrap(middleware::Logger::default())
    });

    env_logger::init();
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
