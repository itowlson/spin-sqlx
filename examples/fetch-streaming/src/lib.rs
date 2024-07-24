use futures::SinkExt;
use futures::stream::StreamExt;
use spin_sdk::http::{Fields, IncomingRequest, OutgoingResponse, ResponseOutparam};
use spin_sdk::http_component;

#[derive(Debug, sqlx::FromRow)]
struct Pet {
    age: i32,
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
async fn handle_fetch_many(_req: IncomingRequest, resp: ResponseOutparam) {
    let og = OutgoingResponse::new(200, &Fields::new(&[
        ("content-type".into(), "text/plain".into())
    ]));
    let mut resp_stm = og.take_body();
    resp.set(og);

    // SQLite
    // let sqlx_conn = spin_sqlx::sqlite::Connection::open_default();
    // let query_sql = "SELECT * FROM pets WHERE age < ?";

    // PostgreSQL
    let sqlx_conn = spin_sqlx::pg::Connection::open("host=localhost user=postgres password=my_password dbname=mydb");
    let query_sql = "SELECT * FROM pets WHERE age < $1";

    let sqlx_conn = match sqlx_conn {
        Ok(c) => c,
        Err(e) => {
            _ = resp_stm.send(format!("{e:?}").into()).await;
            return;
        }
    };

    let pets = sqlx::query_as::<_, Pet>(query_sql)
        .bind(20)
        .fetch(&sqlx_conn);

    let mut resp_lines = pets.map(|pet| match pet {
        Ok(pet) => Ok(format!("{pet}\n").into_bytes()),
        Err(e) => Err(spin_sdk::http::Error::UnexpectedError(format!("{e:?}"))),
    });

    _ = resp_stm.send_all(&mut resp_lines).await;
}
