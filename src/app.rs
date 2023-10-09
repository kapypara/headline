use actix_web::{error, Result, get, post, HttpRequest, HttpResponse, Responder, web};
use actix_files::NamedFile;

use futures::executor::block_on;

mod html;
mod state;
mod db;
mod crypto;

use state::{State, StateMutex};
use db::{Pool, ConnectionManager};

use serde::Deserialize;

pub async fn headline_database() -> Pool {

    log::debug!("making a database connection pool");

    let manager = ConnectionManager::file("db/users.db");
    let pool = Pool::new(manager).unwrap();

    db::user::check(&pool).await;

    pool
}

pub async fn headline_state() -> StateMutex {
    State::new().init().mutex()
}

pub fn headline(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index)
        .service(user_login)
        .service(static_files)
        .service(webfont_files)
        .service(api_user_singup)
        .service(api_user_login);
}

#[get("/")]
async fn index(state: web::Data<StateMutex>, pool: web::Data<Pool>) -> impl Responder {

    let index_file = match read_file("app/index.html").await {
        Some(file) => file,
        None => return Err(error::ErrorInternalServerError("index.html not found"))
    };

    let body = async {
        let state = state.lock().unwrap();
        state.components.navgation_bar.clone() + &index_file + &state.components.footer
    };

    Ok(
        HttpResponse::Ok()
            .body(
                html::HtmlContent {
                    header_tags: None,
                    title: Some("Index Page".to_string()),
                    body: Some(body.await),
                }.to_string()
            )
    )
}

#[get("/login")]
async fn user_login(state: web::Data<StateMutex>, pool: web::Data<Pool>) -> impl Responder {

    let index_file = match read_file("app/login.html").await {
        Some(file) => file,
        None => return Err(error::ErrorInternalServerError("login.html not found"))
    };

    let body = async {
        let state = state.lock().unwrap();
        state.components.navgation_bar.clone() + &index_file + &state.components.footer
    };

    Ok(
        HttpResponse::Ok()
            .body(
                html::HtmlContent {
                    header_tags: None,
                    title: Some("Index Page".to_string()),
                    body: Some(body.await),
                }.to_string()
            )
    )
}


#[get("/static/{filename:.*}")]
async fn static_files(req: HttpRequest) -> actix_web::Result<NamedFile> {

    let request: String = req.match_info().query("filename").parse().unwrap();

    let path: String = "app/static/".to_string() + &request;

    log::trace!("requested static file: {:?}", path);

    Ok(NamedFile::open(path)?)
}

// Font awesome chicanery
#[get("/webfonts/{filename:.*}")]
async fn webfont_files(req: HttpRequest) -> actix_web::Result<NamedFile> {

    let request: String = req.match_info().query("filename").parse().unwrap();

    let path: String = "app/static/webfonts/".to_string() + &request;

    log::trace!("requested static file: {:?}", path);

    Ok(NamedFile::open(path)?)
}


#[derive(Deserialize, Debug)]
struct UserAuth {
    username: Box<str>,
    password: Box<str>,
}

#[post("/api/user_login")]
async fn api_user_login(pool: web::Data<Pool>, info: web::Json<UserAuth>) -> Result<String> {

    let conn = db::get_connection(&pool).await;

    let user = db::user::query_user_by_username(&conn, &info.username).await
        .map_err(error::ErrorInternalServerError)?;

    let user = match user.first() {
        Some(u) => u,
        None => return Err(error::ErrorInternalServerError("Username not found"))
    };

    // let check = *info.password == *user.password;
    let check = crypto::verify_password(&*info.password, &*user.password)
        .map_err(error::ErrorInternalServerError)?;


    if check {
        Ok(format!("Welcome {}!", info.username))
    } else {
        Err(
            error::ErrorUnauthorized(
                format!("Invalid passowrd mister: {}!", info.username)
            )
        )
    }
}


#[post("/api/user_signup")]
async fn api_user_singup(pool: web::Data<Pool>, info: web::Json<UserAuth>) -> Result<String> {

    let conn = db::get_connection(&pool).await;

    let name_is_used = db::user::check_for_username(&conn, &*info.username).await
        .map_err(error::ErrorInternalServerError)?;

    if name_is_used == true {
        return Err(error::ErrorConflict("Username already taken"))
    } 

    let password = crypto::hash_password(&*info.password)
        .map_err(error::ErrorInternalServerError)?;

    let user = db::user::User {
        id: 0,
        username: info.username.clone().into(),
        password: password.into(),
    };

    let user_is_inserted = db::user::insert_new_user(&conn, &user).await
        .map_err(error::ErrorInternalServerError)?;

    if user_is_inserted {
        Ok(format!("Hello to our new use: {}!", info.username))
    } else {
        Err(error::ErrorInternalServerError("Something realy bad happend :("))
    }
}

impl State {

    fn init(mut self) -> Self {

        log::debug!("Making a new state instance");

        block_on(
            async {
                match read_file("app/nav_header.html").await {
                    Some(nav) => self.components.navgation_bar = nav,
                    None => log::warn!("failed to load navgation bar, Bad stuff ahead"),
                }

                match read_file("app/footer.html").await {
                    Some(nav) => self.components.footer = nav,
                    None => log::warn!("failed to load footer :("),
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

#[allow(dead_code)]
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

