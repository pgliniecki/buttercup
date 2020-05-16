#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Json, Path};
use serde_json::Value;

use crate::app::arguments::ArgumentDefinition;
use crate::app::arguments::extraction::{ArgumentsExtractionInput, ArgumentValuesExtractionService};
use crate::app::transformations::transformer::TransformationService;
use crate::app::values::{ValuesPayload, ValueType};
use crate::app::values::extractors::ValueExtractionPolicy;
use crate::builder::content_pipeline_service;

mod app;
mod builder;
mod endpoints;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    TransformationService::initialize();
    let pipeline_service = content_pipeline_service();
    HttpServer::new(move || {
        App::new()
            .data(pipeline_service.clone())
            .service(endpoints::pipeline)
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}