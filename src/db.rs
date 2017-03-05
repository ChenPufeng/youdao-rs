extern crate rusqlite;
extern crate time;
use time::Timespec;
use rusqlite::Connection;

pub fn db_init() {
    let conn = Connection::open("dict.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS youdao_tb(
                  id              INTEGER PRIMARY KEY,
                  word            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  time_updated    TEXT NOT NULL,
                  trans            BLOB
                  )", &[]).unwrap();

}
