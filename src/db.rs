use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, std::io::Error> {
    let statement = client
        .prepare("select * from todo_list")
        .await.unwrap();

    let todos = client
        .query(&statement, &[])
        .await.expect("Error fetching todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}