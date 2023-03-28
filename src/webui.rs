use actix_cors::Cors;
use actix_web::{
    guard,
    http::header::HOST,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

use crate::{
    graphql::{
        schema::{Mutation, Query, Subscription},
        SuperviseurSchema,
    },
    superviseur::core::{ProcessEvent, Superviseur, SuperviseurCommand},
    types::{configuration::ConfigurationData, process::Process},
};

#[derive(RustEmbed)]
#[folder = "webui/build/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

async fn _index_spa() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::post("/graphql")]
async fn index_graphql(
    schema: web::Data<SuperviseurSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::get("/graphiql")]
async fn index_graphiql(req: HttpRequest) -> Result<HttpResponse> {
    let host = req
        .headers()
        .get(HOST)
        .unwrap()
        .to_str()
        .unwrap()
        .split(":")
        .next()
        .unwrap();

    const PORT: u16 = 5478;
    let graphql_endpoint = format!("http://{}:{}/graphql", host, PORT);
    let ws_endpoint = format!("ws://{}:{}/graphql", host, PORT);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint(&graphql_endpoint)
                .subscription_endpoint(&ws_endpoint)
                .finish(),
        ))
}

async fn index_ws(
    schema: web::Data<SuperviseurSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

pub async fn start_webui(
    config_file_path: String,
    cmd_tx: mpsc::UnboundedSender<SuperviseurCommand>,
    event_tx: mpsc::UnboundedSender<ProcessEvent>,
    superviseur: Superviseur,
    processes: Arc<Mutex<Vec<(Process, String)>>>,
    config_map: Arc<Mutex<HashMap<String, ConfigurationData>>>,
) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", 5478);

    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(config_file_path)
    .data(superviseur)
    .data(cmd_tx)
    .data(event_tx)
    .data(processes)
    .data(config_map)
    .finish();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(cors)
            .service(index_graphql)
            .service(index_graphiql)
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(index)
            .service(dist)
    })
    .bind(addr)?
    .run()
    .await
}
