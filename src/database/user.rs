use crate::database::Database;
use mysql::params;
use mysql::prelude::Queryable;

pub struct User;

impl User {
    pub fn update_password(db: &Database, login: &Option<String>, new_password: &String) {
        if let Some(login) = login {
            let sql = "UPDATE akteur SET password = SHA2(:new_password, 256) \
                WHERE login = :login";
            let _ = db.connection().exec_drop(
                sql,
                params! {"login" => login, "new_password" => new_password},
            );
        }

        let sql = "UPDATE akteur SET password = SHA2(:new_password, 256)";
        let _ = db
            .connection()
            .exec_drop(sql, params! {"new_password" => new_password});
    }
}
