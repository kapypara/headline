use rusqlite::{Result, Error};
pub use r2d2_sqlite::{self, SqliteConnectionManager};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

type QueryResult<T> = Result<Vec<T>, Error>;

#[derive(Clone, Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub pass: String,
    pub salt: String
}

pub async fn get_connection(pool: &Pool) -> Connection {
    // TODO: handle error
    let pool = pool.clone();
    pool.get().unwrap()
}

pub async fn check_if_table_exists(conn: &Connection, name: &str) -> Result<bool> {
    conn.query_row(
        "SELECT EXISTS ( SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1 )",
        [name],
        |row| row.get(0),
    )
}

pub async fn create_user_table(conn: &Connection) -> Result<usize> {

    let stmnt = "
CREATE TABLE IF NOT EXISTS user (
    id   INTEGER PRIMARY KEY,
    name TEXT    NOT NULL,
    pass TEXT    CHECK(length(pass) > 0),
    salt TEXT    CHECK(length(salt) > 0)
)";

    conn.execute(stmnt, [])
}

pub async fn query_username(conn: &Connection, name: &str) -> QueryResult<User> {

    let stmnt = "SELECT * FROM user WHERE name = ?1";

    conn.prepare(stmnt)?.query_map(
        [name],
        |row| {
            Ok(User{
                id: row.get(0)?,
                name: row.get(1)?,
                pass: row.get(2)?,
                salt: row.get(3)?
            })
        })?.collect()
}                       

pub async fn check(pool: &Pool) {

    let conn = get_connection(pool).await;

    let table_exists = match check_if_table_exists(&conn, "user").await {
        Ok(bool) => bool,
        Err(err) => {
            log::error!("failed to check if user table exists, go error: {}", err);
            return
        }

    };

    if table_exists {
        log::debug!("user table does exists");
        return

    }

    log::warn!("user table don't exists, creating it");

    match create_user_table(&conn).await {
        Err(err) => log::error!("failed at creating user table, go error {}", err),
        _ => {}
    };

    match check_if_table_exists(&conn, "user").await {
        Ok(exists) => {
            if exists {
                log::debug!("successfully created user table");
            } else {
                log::debug!("failed at checking for user table")
            }
        }

        Err(err) => {
            log::error!("couldn't check the user table exists after creation attempt, got error: {}", err)
        }
    }
}

