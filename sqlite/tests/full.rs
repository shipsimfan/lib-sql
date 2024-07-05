use sqlite::{
    sql::{Connection, Statement},
    SQLite3Connection,
};

/// Tests a variety of functions to make sure they work
#[test]
fn full() {
    let connection = SQLite3Connection::open(":memory:").unwrap();

    connection
        .execute("CREATE TABLE test(id INTEGER PRIMARY KEY, name VARCHAR(64) NOT NULL); INSERT INTO test (name) VALUES (\"Testing\");")
        .unwrap();

    let mut statement = connection
        .prepare("SELECT name FROM test WHERE id = ?")
        .unwrap();

    statement.bind(1, &1).unwrap();

    let row: String = statement.rows().unwrap().next().unwrap().unwrap();
    assert_eq!(row, "Testing");

    drop(connection);
}
