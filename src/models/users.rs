use crate::models::connect_db;
use crate::schema::users;
use diesel::prelude::*;
use diesel::{AsChangeset, Identifiable, Insertable, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use std::error::Error;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub async fn get_all() -> Result<Vec<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db().await?;

        Ok(users.load::<User>(&mut conn).await?)
    }

    pub async fn get_by_name(u_name: &str) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db().await?;

        match users.filter(name.eq(u_name)).first::<User>(&mut conn).await {
            Ok(x) => Ok(Some(x)),
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(err.into()),
            },
        }
    }

    pub async fn get_by_id(u_id: &str) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db().await?;

        match users.filter(id.eq(u_id)).first::<User>(&mut conn).await {
            Ok(x) => Ok(Some(x)),
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    return Ok(None);
                }
                _ => return Err(err.into()),
            },
        }
    }

    // pub async fn get_by_tele_id(u_id: &str) -> Result<Option<User>, Box<dyn Error>> {
    //     use crate::schema::users::dsl::*;

    //     let mut conn = connect_db().await?;

    //     match users
    //         .filter(telegram_id.eq(u_id))
    //         .first::<User>(&mut conn)
    //         .await
    //     {
    //         Ok(x) => Ok(Some(x)),
    //         Err(err) => match err {
    //             diesel::result::Error::NotFound => {
    //                 return Ok(None);
    //             }
    //             _ => return Err(err.into()),
    //         },
    //     }
    // }

    pub async fn update(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = connect_db().await?;

        diesel::update(&self).set(self).execute(&mut conn).await?;

        Ok(())
    }

    pub async fn delete(self) -> Result<(), Box<dyn Error>> {
        let mut conn = connect_db().await?;

        diesel::delete(&self).execute(&mut conn).await?;

        Ok(())
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub id: String,
    pub name: String,
}

impl NewUser {
    pub async fn save(&self) -> Result<(), Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let mut conn = connect_db().await?;

        diesel::insert_into(users)
            .values(self)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use std::future::Future;

    use super::*;
    use crate::models::test::test_connect_db;
    use diesel_async::AsyncPgConnection;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_user(
        _test_connect_db: impl Future<Output = AsyncPgConnection>,
    ) -> Result<(), Box<dyn Error>> {
        let name = "012345678901234567890123456789012345678901234567890123456789";
        let new_name = "111111111111111111111111111111111111111111111111111111111111";

        // create a new user
        let u1 = NewUser {
            id: "123456789".to_owned(),
            name: name.to_owned(),
        };

        // save the user
        assert!(u1.save().await.is_ok());

        // get the user to check if it exists
        assert!(User::get_by_name(name).await.is_ok());

        // update the user's name
        let mut u2 = User::get_by_name(name).await?.unwrap();
        u2.name = new_name.to_owned();
        assert!(u2.update().await.is_ok());

        // delete the user
        assert!(User::get_by_name(new_name)
            .await?
            .unwrap()
            .delete()
            .await
            .is_ok());

        Ok(())
    }

    #[fixture]
    pub async fn create_sample_users() -> Vec<User> {
        use rand::Rng;
        use rnglib::{Language, RNG};

        let rng = RNG::new(&Language::Roman).unwrap();

        let mut res: Vec<User> = Vec::new();

        for _ in 0..2 {
            let name = format!(
                "{}-{}-{}",
                rng.generate_name(),
                rng.generate_name(),
                rand::thread_rng().gen_range(0..100)
            );

            let u1 = NewUser {
                id: format!(
                    "{}{}",
                    rand::thread_rng().gen_range(100..999),
                    rand::thread_rng().gen_range(1000..9999)
                ),
                name: name.clone(),
            };

            u1.save().await.unwrap();

            if let Ok(Some(u)) = User::get_by_name(&u1.name).await {
                res.push(u);
            } else {
                panic!("Unable to add users sample users to db...")
            }
        }

        return res;
    }
}
