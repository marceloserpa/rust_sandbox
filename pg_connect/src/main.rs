extern crate postgres;
extern crate rand;

use rand::Rng;
use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {

    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0,50000);

    let conn = Connection::connect("postgres://postgres:docker@localhost:5432", TlsMode::None).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string() + " " + &number.to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}", person.name);
    }
}
