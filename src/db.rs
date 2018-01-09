
use std::path::Path;

use rusqlite::{Connection, Result};

use super::result::{PingResult, PingStatus};

/// A database connection
pub struct Database {
    connection: Connection,
}

/// The SQL used to create the table
const CREATE_SQL: &str = "CREATE TABLE IF NOT EXISTS `results` ( \
    `time` TEXT NOT NULL, \
    `returned` INTEGER NOT NULL, \
    `time_s` REAL \
    CHECK ( ( returned = 0 AND time_s IS NULL ) OR ( returned = 1 AND time_s IS NOT NULL )) )";

/// The SQL used to insert a row
const INSERT_SQL: &str = "INSERT INTO `results` VALUES ( ?, ?, ? )";

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = Connection::open(path)?;

        // Check schema
        connection.execute(CREATE_SQL, &[])?;

        Ok(Database { connection })
    }

    pub fn save_result(&mut self, result: PingResult) -> Result<()> {
        let time_str = result.time().to_rfc3339();
        let (returned, time_ms): (i32, Option<f64>) = match result.status() {
            PingStatus::Returned(time_s) => (1, Some(time_s)),
            PingStatus::Timeout => (0, None),
        };

        self.connection.execute(INSERT_SQL, &[&time_str, &returned, &time_ms])?;
        Ok(())
    }

}
