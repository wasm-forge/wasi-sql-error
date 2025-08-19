fn main() {
    // delete old database to have a clean experiment
    let _ = std::fs::remove_file("main.db");
    let _ = std::fs::remove_file("main.db-journal");

    // Create a new connection to the database
    let mut conn = rusqlite::Connection::open("main.db").unwrap();

    // >>> this is a workaround to avoid the error
    conn.execute("PRAGMA temp_store = MEMORY", ()).unwrap();

    // create some table
    conn.execute("CREATE TABLE persons (name, email)", ())
        .unwrap();

    // add 1000000 rows of data
    {
        // start transaction
        let tx = conn.transaction().unwrap();

        let mut stmt = tx
            .prepare_cached("INSERT INTO persons (name, email) VALUES (?, ?)")
            .unwrap();

        for i in 0..1000000 {
            let name = format!("name{i}");
            let email = format!("name{i}@example.com");

            stmt.execute(rusqlite::params![name, email]).unwrap();
        }

        drop(stmt);

        tx.commit().unwrap();
    }

    // >>> create index (fails when temp_store is inside a file)
    conn.execute("CREATE INDEX idx_email ON persons(email)", ())
        .unwrap();
}
