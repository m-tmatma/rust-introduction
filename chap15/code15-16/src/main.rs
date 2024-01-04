use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    let cn = Connection::open_in_memory()?;
    cn.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)", params![])?;

    let mut stmt = cn.prepare("INSERT INTO users (id, name, age) VALUES (?, ?, ?)")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Hieai", 20])?;
    stmt.execute(params![3, "Haruna", 18])?;
    stmt.execute(params![4, "Kirishima", 15])?;

    let mut stmt = cn.prepare("SELECT * FROM users")?;
    let mut rows = stmt.query(params![])?;
    while let Some(rows) = rows.next()? {
        let id: i32 = rows.get(0)?;
        let name: String = rows.get(1)?;
        let age: i32 = rows.get(2)?;
        println!("id: {}, name: {}, age: {}", id, name, age);
    }
    Ok(())
}
