use std::io::{Read, Write};
use tiny_http::{Request, Response, Server};

fn handle_request(request: Request) {
    // Lógica de manipulação do pedido aqui
    let response = Response::from_string("Hello, world!");

    if let Err(err) = request.respond(response) {
        eprintln!("Erro ao responder à solicitação: {}", err);
    }
}

fn main() {
    let server = Server::http("127.0.0.1:8080").expect("Erro ao iniciar o servidor.");

    println!("Servidor HTTP rodando em http://127.0.0.1:8080");

    for request in server.incoming_requests() {
        handle_request(request);
    }
}

