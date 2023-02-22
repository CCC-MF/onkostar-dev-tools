pub mod markmalskatalog;
pub mod patient;
pub mod user;

use mysql::{Pool, PooledConn};

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
    ) -> Database {
        let url = format!("mysql://{username}:{password}@{host}:{port}/{name}");
        let pool = Pool::new(url.as_str()).expect("connection pool");
        Database { pool }
    }

    fn connection(&self) -> PooledConn {
        self.pool.get_conn().expect("connection")
    }
}
