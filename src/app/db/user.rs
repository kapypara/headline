use super::{
    get_connection, check_if_table_exists,
    Connection, Pool,
    Result, QueryResult
};

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct User {
    pub id: usize,
    pub username: Arc<str>,
    pub password: Arc<str>,
}

pub async fn create_table(conn: &Connection) -> Result<usize> {

    let stmnt = "
CREATE TABLE IF NOT EXISTS user (
    id       INTEGER PRIMARY KEY,
    username TEXT    UNIQUE NOT NULL,
    password TEXT    CHECK(length(password) > 0)
)";

    conn.execute(stmnt, [])
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

    match create_table(&conn).await {
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

/// return true if a user was inserted
pub async fn insert_new_user(conn: &Connection, user: &User) -> Result<bool> {

    let stmnt = "INSERT INTO user (username, password) VALUES (?1, ?2)";

    Ok(
        conn.execute(
            stmnt,
            [&user.username, &user.password]
        )? != 0
    )
}

/// TODO return a result<User> isntead of QueryResult<User>
pub async fn query_user_by_username(conn: &Connection, name: &str) -> QueryResult<User> {

    let stmnt = "SELECT * FROM user WHERE username = ?1";

    conn.prepare(stmnt)?.query_map(
        [name],
        |row| {
            Ok(User{
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })?.collect()
}


/// return true when username is already exists
pub async fn check_for_username(conn: &Connection, name: &str) -> Result<bool> {

    let namecheck = query_user_by_username(conn, name).await;

    Ok(namecheck?.len() != 0)
}
