use actix_web::{get, HttpResponse, Responder, web};
use actix_files::NamedFile;

use futures::executor::block_on;

mod html;
mod state;

type AppState = web::Data<state::State>;

pub fn headline(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index)
        .service(css)
        .app_data(
            web::Data::new(
                init_state()
            )
        );
}


#[get("/")]
async fn index(state: AppState) -> impl Responder {

    let index_file = match read_file("app/index.html").await {
        Some(file) => file,
        None => return HttpResponse::InternalServerError().body("index.html not found")
    };

    let body = state.components.navgation_bar.clone() + &index_file;


    HttpResponse::Ok()
        .body(
            html::HtmlContent {
                header_tags: None,
                title: Some("Index Page".to_string()),
                body: Some(body),
            }.to_string()
        )
}


#[get("/bulma.css")]
async fn css() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("app/bulma.css")?)
}

fn init_state() -> state::State {

    let mut s = state::State::new();

    block_on(
        async {
            match read_file("app/nav_header.html").await {
                Some(nav) => s.components.navgation_bar = nav,
                None => log::error!("failed to load navgation bar, Bad stuff ahead"),
            }
        });

    s
}

async fn read_file(path: &str) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(file) => Some(file),
        Err(err) => {
            log::error!("tried to read: {}, got error: {}", path, err);
            return None
        }
    }
}

async fn load_markdown_from_file(path: &str) -> Option<String> {

    let file = read_file(path);

    let options = &markdown::Options {
        compile: markdown::CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..markdown::CompileOptions::default()
        },
        ..markdown::Options::default()
    };

    let file = match file.await {
        Some(file) => file,
        None => return None
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

