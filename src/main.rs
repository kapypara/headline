use actix_web::{
    get, post, web, App, Error, error,
    HttpResponse, HttpServer,
    middleware,
    http::{header::ContentType, StatusCode}, HttpRequest,
    body::{BoxBody, self},
    Responder, Result
};

use markdown;

use actix_files::NamedFile;

// use std::path::PathBuf;

// use log::{info};
// use json;
use serde::Serialize;

use futures::{future::ok, stream::once};

use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize)]
struct VisitorCounter {
    counter: Mutex<u64>,
}

// Responder
impl Responder for VisitorCounter {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        let body = once(ok::<_, Error>(web::Bytes::from(body)));

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .streaming(body)
    }
}

async fn read_file(path: &str) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

async fn load_markdown_from_file(path: &str) -> Option<String> {

    let file = async {
        std::fs::read_to_string(path)
    };

    let options = &markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..markdown::CompileOptions::default()
        },
        ..markdown::Options::default()
    };

    let file = match file.await {
        Ok(file) => file,
        Err(err) => {
            log::error!("tried to read: {}, got error: {}", path, err);
            return None
        }
    };

    match markdown::to_html_with_options(&file, options) {
        Ok(val) => {
            log::trace!("markdown file: {}, to html: {}", path, val);
            Some(val)
        },

        Err(err) => {
            log::error!("faild to convert markdown to html. path: {}, error: {}", path, err);
            None
        }
    }
}

#[derive(Clone, Debug)]
struct HtmlContent {
    header_tags: Option<HashMap<String, String>>,
    title: Option<String>,
    body: Option<String>,
}

impl HtmlContent {
    fn to_string(self) -> String {

        log::trace!("stringifing HtmlContent: {:?}", self);

        let header_start = r#"<!doctype html>
<html lang="en-US">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css">
"#;

        let header_end = "</head>\n";

        let title_content = match self.title {
            Some(title) => title,
            None => "HEADLINE SERVER".to_string(),
        };
        let title = "    <title>".to_string() + &title_content + "</title>\n";

        let header = header_start.to_string() + &title + header_end;


        let body_content =  match self.body {
            Some(body) => body,
            None => "NO PAGE BODY".to_string()
        };
        let body = "<body>\n".to_string() + &body_content + "</body>";

        header + &body + "\n</html>"
    }
}

#[get("/")]
async fn index<'a>(data : web::Data<VisitorCounter>) -> impl Responder {

    let index_file = read_file("app/index.html");

    let mut counter = data.counter.lock().unwrap();
    *counter += 1;


    HttpResponse::Ok()
        .body(
            HtmlContent {
                header_tags: None,
                title: None,
                body: index_file.await,
            }.to_string()
        )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("{tail:.*}")]
async fn app_home() -> Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // std::env::set_var("RUST_LOG", "info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    log::info!("starting HTTP server at http://localhost:8080");

    let counter = web::Data::new(VisitorCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(counter.clone())
            .service(index)
            .service(echo)
            .service(manual_hello)
            .service(web::scope("/app").service(app_home))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
    .await
}
