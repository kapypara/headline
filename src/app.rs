use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_files::NamedFile;

use std::sync::Mutex;
use futures::executor::block_on;

mod html;
mod state;

type AppState = web::Data<Mutex<state::State>>;

pub fn headline(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index)
        .service(static_files)
        .app_data(
            web::Data::new(
                Mutex::new(state::State::new().init())
            )
        );
}


#[get("/")]
async fn index(state: AppState) -> impl Responder {

    let index_file = match read_file("app/index.html").await {
        Some(file) => file,
        None => return HttpResponse::InternalServerError().body("index.html not found")
    };

    let body = async {
        let state = state.lock().unwrap();
        state.components.navgation_bar.clone() + &index_file + &state.components.footer
    };


    HttpResponse::Ok()
        .body(
            html::HtmlContent {
                header_tags: None,
                title: Some("Index Page".to_string()),
                body: Some(body.await),
            }.to_string()
        )
}


#[get("/static/{filename:.*}")]
async fn static_files(req: HttpRequest) -> actix_web::Result<NamedFile> {

    let request: String = req.match_info().query("filename").parse().unwrap();

    let path: String = "app/static/".to_string() + &request;

    log::trace!("requested static file: {:?}", path);

    Ok(NamedFile::open(path)?)
}

impl state::State {

    fn init(mut self) -> Self {
        block_on(
            async {
                match read_file("app/nav_header.html").await {
                    Some(nav) => self.components.navgation_bar = nav,
                    None => log::error!("failed to load navgation bar, Bad stuff ahead"),
                }

                match read_file("app/footer.html").await {
                    Some(nav) => self.components.footer = nav,
                    None => log::error!("failed to load footer :("),
                }
            });

        self
    }
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

