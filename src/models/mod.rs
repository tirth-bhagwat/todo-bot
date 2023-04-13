pub mod todos;
pub mod users;

use std::{env, error::Error};

use diesel_async::{AsyncConnection, AsyncPgConnection};

use diesel::result::Error::{DatabaseError, NotFound};

pub enum DbError {
    NotFound,
    Other,
}

pub async fn connect_db() -> Result<AsyncPgConnection, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    Ok(AsyncPgConnection::establish(&database_url).await?)
}

fn fmt_error(err: diesel::result::Error) -> DbError {
    match err {
        NotFound => DbError::NotFound,
        _ => DbError::Other,
    }
}

#[cfg(test)]
pub mod test {
    use diesel_async::AsyncPgConnection;
    use rstest::*;

    #[fixture]
    pub async fn test_connect_db() -> AsyncPgConnection {
        match super::connect_db().await {
            Ok(conn) => conn,
            Err(err) => {
                dbg!(err);
                panic!("Error while connecting to database...");
            }
        }
    }
}
