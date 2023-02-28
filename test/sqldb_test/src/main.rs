use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

async fn run() -> Result<()> {
    let conn = Connection::open("test.db")?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
    };

    conn.execute(
        "INSERT INTO person (name) VALUES (?1)",
        [me.name],
    )?;

    let mut stmt = conn.prepare("SELECT id, name FROM person")?;

    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_test() {
        let conn = Connection::open("test.db").unwrap();
        assert_eq!(conn.is_autocommit(), true);
        println!("Connected to database");
    }
}
