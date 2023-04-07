use crate::models::connect_db;
use crate::schema::users;
use diesel::prelude::*;
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable};
use std::error::Error;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug)]
struct User {
    id: i32,
    name: String,
}

impl User {
    fn get_all() -> Result<Vec<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db()?;

        Ok(users.load::<User>(&mut conn)?)
    }

    fn get_by_name(u_name: &str) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db()?;

        match users.filter(name.eq(u_name)).first::<User>(&mut conn) {
            Ok(x) => Ok(Some(x)),
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(err.into()),
            },
        }
    }

    fn get_by_id(u_id: i32) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db()?;

        match users.filter(id.eq(u_id)).first::<User>(&mut conn) {
            Ok(x) => Ok(Some(x)),
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(err.into()),
            },
        }
    }

    fn update(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = connect_db()?;

        diesel::update(&self).set(self).execute(&mut conn)?;

        Ok(())
    }

    fn delete(self) -> Result<(), Box<dyn Error>> {
        let mut conn = connect_db()?;

        diesel::delete(&self).execute(&mut conn)?;

        Ok(())
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
struct NewUser {
    name: String,
}

impl NewUser {
    fn save(&self) -> Result<(), Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db().unwrap();

        diesel::insert_into(users).values(self).execute(&mut conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_user() -> Result<(), Box<dyn Error>> {
        assert!(connect_db().is_ok());

        let name = "012345678901234567890123456789012345678901234567890123456789";
        let new_name = "111111111111111111111111111111111111111111111111111111111111";

        // create a new user
        let u1 = NewUser {
            name: name.to_owned(),
        };

        // save the user
        assert!(u1.save().is_ok());

        // get the user to check if it exists
        assert!(User::get_by_name(name).is_ok());

        // update the user's name
        let mut u2 = User::get_by_name(name)?.unwrap();
        u2.name = new_name.to_owned();
        assert!(u2.update().is_ok());

        // delete the user
        assert!(User::get_by_name(new_name)?.unwrap().delete().is_ok());

        Ok(())
    }
}
