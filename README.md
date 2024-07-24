# spin-sqlx

A partial implementation of sqlx for Fermyon Spin's SQLite database and PostgreSQL API.

Things that work (at least in my one, super simple, test case!):

* Standard fetching and execution functions

Things that don't:

* Typed queries
* Prepared statements
* Logging
* Error handling, really. Sorry

Example (SQLite):

```rust
// CREATE TABLE pets (age INTEGER, name TEXT, is_finicky BOOL, real_thingy REAL, blobbles BINARY);

#[derive(Debug, sqlx::FromRow)]
struct Pet {
    age: u32,
    name: String,
    is_finicky: bool,
    real_thingy: f32,
    blobbles: Vec<u8>,
}

#[http_component]
async fn handle_sqlxtest(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let sqlx_conn = spin_sqlx::sqlite::Connection::open_default()?;

    let pets: Vec<Pet> = sqlx::query_as("SELECT * FROM pets WHERE age < ?")
        .bind(7)
        .fetch_all(&sqlx_conn)
        .await?;
    let resp = pets.iter().map(|p| format!("{p}")).collect::<Vec<_>>().join(" and ");

    Ok(spin_sdk::http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(resp)
        .build())
}
```
