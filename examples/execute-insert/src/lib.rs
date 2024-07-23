use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
async fn handle_execute_insert(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let sqlx_conn = spin_sqlx::sqlite::Connection::open_default()?;

    sqlx::query("INSERT INTO pets(age, name, is_finicky) VALUES (?, ?, ?)")
        .bind(1)
        .bind("Rosie")
        .bind(true)
        .execute(&sqlx_conn)
        .await?;

    let (count,): (u32,) = sqlx::query_as("SELECT COUNT(*) FROM pets")
        .fetch_one(&sqlx_conn)
        .await?;
    let response = format!("The database now contains {count} pets");
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(response)
        .build())
}
