use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    let cn = Connection::open_in_memory()?;

    cn.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)", params![])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (?, ?, ?)", params![1, "Kongo", 20])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (?, ?, ?)", params![1, "Hieai", 20])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (?, ?, ?)", params![1, "Haruna", 18])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (?, ?, ?)", params![1, "Kirishima", 15])?;
    Ok(())
}
