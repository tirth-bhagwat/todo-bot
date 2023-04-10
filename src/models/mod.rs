use std::{env, error::Error};

use diesel::{Connection, PgConnection};

pub mod todos;
pub mod users;

fn connect_db() -> Result<PgConnection, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    Ok(PgConnection::establish(&database_url)?)
}

#[cfg(test)]
pub mod test {
    use diesel::PgConnection;
    use rstest::*;

    #[fixture]
    pub fn test_connect_db() -> PgConnection {
        match super::connect_db() {
            Ok(conn) => conn,
            Err(err) => {
                dbg!(err);
                panic!("Error while connecting to database...");
            }
        }
    }
}
