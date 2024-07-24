use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[http_component]
async fn handle_execute_insert(_req: Request) -> anyhow::Result<impl IntoResponse> {
    // let sqlx_conn = spin_sqlx::sqlite::Connection::open_default()?;
    // let query_sql = "INSERT INTO pets(age, name, is_finicky) VALUES (?, ?, ?)";
    let sqlx_conn = spin_sqlx::pg::Connection::open("host=localhost user=postgres password=my_password dbname=mydb")?;
    let query_sql = "INSERT INTO pets(age, name, is_finicky) VALUES ($1, $2, $3)";

    // SQLite
    // sqlx::query("INSERT INTO pets(age, name, is_finicky) VALUES (?, ?, ?)")
    //     .bind(1)
    //     .bind("Rosie")
    //     .bind(true)
    //     .execute(&sqlx_conn)
    //     .await?;

    // let (count,): (u32,) = sqlx::query_as("SELECT COUNT(*) FROM pets")
    //     .fetch_one(&sqlx_conn)
    //     .await?;
    // let response = format!("The database now contains {count} pets");

    // PostgreSQL
    let affected = sqlx::query(query_sql)
        .bind(1)
        .bind("Rosie")
        .bind(true)
        .execute(&sqlx_conn)
        .await?
        .count();

    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM pets")
        .fetch_one(&sqlx_conn)
        .await?;
    let response = format!("The database now contains {count} pets, of which {affected} were changed in the insert");
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(response)
        .build())
}
