use std::{ env };

use engine::connection::{ Connection, Config };
use log::{ info, error };

const DEFAULT_DB_PATH: &str = "./db/database.data";
fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    // We expect the first arg to be the database path
    let database_path = args.get(1).get_or_insert(&DEFAULT_DB_PATH.to_string()).clone();

    let connection_result = Connection::open(database_path, Config {
        ..Default::default()
    });

    if connection_result.is_err() {
        error!("Connection to database cannot be established: {}", connection_result.unwrap_err());
    } else {
        info!("Connection to database engine is succesfully established");
        let mut connection = connection_result.unwrap();

        let random_data = vec!["K".as_bytes()[0]; 256];

        for i in 1..2000 {
            connection.insert(i, &random_data).unwrap();
        }

        connection.insert(0, "Hello world!".as_bytes()).unwrap();
    }
}