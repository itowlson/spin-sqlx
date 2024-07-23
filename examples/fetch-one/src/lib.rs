use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(Debug, sqlx::FromRow)]
struct Pet {
    age: u32,
    name: String,
    is_finicky: bool,
}

impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fin_desc = if self.is_finicky { "is" } else { "is not" };
        f.write_fmt(format_args!("{}, aged {}, {} finicky", self.name, self.age, fin_desc))
    }
}

#[http_component]
async fn handle_fetch_one(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let sqlx_conn = spin_sqlx::sqlite::Connection::open_default()?;

    let pet: Pet = sqlx::query_as("SELECT * FROM pets WHERE name = ?")
        .bind("Slats")
        .fetch_one(&sqlx_conn)
        .await?;

    let response = format!("{pet}");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(response)
        .build())
}
