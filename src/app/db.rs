use rusqlite::{Result, Error};
use r2d2_sqlite::{self, SqliteConnectionManager};

pub mod user;

pub type ConnectionManager = SqliteConnectionManager;
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

type QueryResult<T> = Result<Vec<T>, Error>;

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


