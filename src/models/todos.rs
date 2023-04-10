use crate::models::connect_db;
use crate::schema::todos;
use diesel::prelude::*;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, RunQueryDsl, Selectable};
use std::error::Error;

use super::users::User;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Associations, Debug)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub status: i32,
}

impl Todo {
    pub fn get_by_title(t_title: &str) -> Result<Vec<Todo>, Box<dyn Error>> {
        use crate::schema::todos::dsl::*;

        let mut conn = connect_db()?;

        Ok(todos
            .filter(title.like(format!("{}%", t_title)))
            .load::<Todo>(&mut conn)?)
    }

    pub fn get_for_user(u_id: i32) -> Result<Vec<(Todo, User)>, Box<dyn Error>> {
        use crate::schema::users;

        let mut conn = connect_db()?;

        Ok(todos::table
            .inner_join(users::table)
            .filter(users::id.eq(u_id))
            .load::<(Todo, User)>(&mut conn)?)
    }

    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        use crate::schema::todos::dsl::*;

        let mut conn = connect_db()?;

        diesel::update(todos)
            .filter(id.eq(self.id))
            .set(self)
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn delete(self) -> Result<(), Box<dyn Error>> {
        use crate::schema::todos::dsl::*;

        let mut conn = connect_db()?;

        diesel::delete(todos)
            .filter(id.eq(self.id))
            .execute(&mut conn)?;

        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "todos"]
struct NewTodo {
    title: String,
    description: Option<String>,
    user_id: i32,
    status: i32,
}

impl NewTodo {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        use crate::schema::todos::dsl::*;

        let mut conn = connect_db()?;

        diesel::insert_into(todos).values(self).execute(&mut conn)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::users::{test::create_sample_users, User};
    use rstest::*;
    use std::error::Error;

    #[rstest]
    fn test_todo(
        #[from(create_sample_users)] sample_users: Vec<User>,
    ) -> Result<(), Box<dyn Error>> {
        // create a new todo
        let t1 = NewTodo {
            title: "Todo 12332112311233211231".to_owned(),
            description: Some("Description for todo 12332112311233211231".to_owned()),
            user_id: sample_users.get(0).unwrap().id,
            status: 0,
        };

        // save it
        assert!(t1.save().is_ok(), "Failed to save todo");

        // read the todo
        let mut t1 = Todo::get_by_title("Todo 12332112311233211231").unwrap();
        let mut x = t1.remove(0);

        // update the todo
        x.status = 2;
        assert!(x.update().is_ok(), "Failed to update todo");

        // delete the todo
        assert!(x.delete().is_ok(), "Failed to delete todo");

        // delete the sample users
        for u in sample_users {
            assert!(u.delete().is_ok(), "Failed to delete user");
        }

        Ok(())
    }
}
