use chrono::{DateTime, Local};
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::Migrations;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

struct TodoItem {
    id: u64,
    item: String,
    done: bool,
    added: DateTime<Local>,
    completed: Option<DateTime<Local>>
}

impl TodoItem {
    pub fn get_str(&self) -> String { format!("[{}] {} - {}", self.id, if self.done {"X"} else {" "}, self.item) }
}

pub fn init_database(db_path: &str) -> Connection {
    let mut conn = Connection::open(db_path).expect("Failed to open database connection");
    MIGRATIONS.to_latest(&mut conn).expect("Failed to carry out database migrations");
    conn
}

pub fn add_item(conn: &mut Connection, item: &str) -> String {
    let result = conn.execute("INSERT INTO tasks (item, done, added) VALUES (?1, ?2, DateTime('now'))", (item, false));
    String::from("todo")
}
