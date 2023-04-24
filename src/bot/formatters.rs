use teloxide::utils::markdown::{bold, escape};

use crate::models::todos::Todo;

pub fn format_todo(todo: Todo) -> String {
    let mut todo_string = String::new();
    todo_string.push_str(&format!("{} :\n", bold(&todo.title)));
    if let Some(desc) = todo.description {
        todo_string.push_str(&format!("{}", desc));
    }
    todo_string
}

pub fn format_todos(todos: Vec<Todo>) -> String {
    let mut todos_string = String::new();
    for (i, todo) in todos.into_iter().enumerate() {
        todos_string.push_str(&format!(
            "{}{} {}\n\n",
            i + 1,
            escape(")"),
            format_todo(todo)
        ));
    }
    todos_string
}
