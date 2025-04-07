use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

async fn health_check(_: HttpRequest) -> impl Responder {
    return HttpResponse::Ok().finish();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    return Ok(server);
}
