use mysql::Error::{DriverError, MySqlError};
use mysql::{Pool, PooledConn};

pub mod datenkatalog;
pub mod form;
pub mod merkmalskatalog;
pub mod patient;
pub mod prozedur;
pub mod user;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(
        username: String,
        password: String,
        host: String,
        port: String,
        name: String,
    ) -> Result<Database, String> {
        let url = format!("mysql://{username}:{password}@{host}:{port}/{name}");
        match Pool::new(url.as_str()) {
            Ok(pool) => Ok(Database { pool }),
            Err(e) => {
                let cause = match e {
                    DriverError(e) => e.to_string(),
                    MySqlError(e) => e.to_string(),
                    _ => "Keine weiteren Angaben".to_string(),
                };
                return Err(format!("Keine Datenbankverbindung möglich: {}", cause));
            }
        }
    }

    fn connection(&self) -> PooledConn {
        self.pool.get_conn().expect("connection")
    }
}
