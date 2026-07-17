use postgres::{Client, NoTls};
use std::env::{self, VarError};
use std::error::Error;

pub fn create_client() -> Result<Client, Box<dyn Error>> {
    let postgresql_addon_host = env::var("POSTGRESQL_ADDON_HOST")?;
    let postgresql_addon_db = env::var("POSTGRESQL_ADDON_DB")?;
    let postgresql_addon_user = env::var("POSTGRESQL_ADDON_USER")?;
    let postgresql_addon_port = env::var("POSTGRESQL_ADDON_PORT")?.parse::<u16>()?;
    let postgresql_addon_password = env::var("POSTGRESQL_ADDON_PASSWORD")?;

    let mut config = Client::configure();

    config.host(&postgresql_addon_host);
    config.dbname(&postgresql_addon_db);
    config.user(&postgresql_addon_user);
    config.port(postgresql_addon_port);
    config.password(&postgresql_addon_password);

    Ok(config.connect(NoTls)?)
}

pub fn create_table(client: Result<Client, Box<dyn Error>>) {
    match client {
        Ok(mut client) => {
            let create_table_string = format!(
                r#"
                CREATE EXTENSION IF NOT EXISTS pgcrypto;

                CREATE TABLE IF NOT EXISTS posts (
                  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
                  title text,
                  body text,
                  date text
                )
              "#
            );
            let response = client.batch_execute(&create_table_string);
            match response {
                Ok(_) => println!("posts table exists!"),
                Err(error) => println!("Error! {error}"),
            }
        }
        Err(error) => println!("There was a DB error! {error}"),
    }
}
