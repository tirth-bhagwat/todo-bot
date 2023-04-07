use std::{env, error::Error};

use diesel::{Connection, PgConnection};

pub mod users;

fn connect_db() -> Result<PgConnection, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    Ok(PgConnection::establish(&database_url)?)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_connect_db() {
        assert!(super::connect_db().is_ok());
    }
}
