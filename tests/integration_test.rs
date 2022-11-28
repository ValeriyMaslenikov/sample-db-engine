



/// The purpose of these tests is to do tree validation after the execution of multiple operations.

#[cfg(feature = "integration")]
mod helpers {
    use lazy_static::lazy_static;
    use uuid::Uuid;

    use super::*;

    lazy_static! {
        static ref TEST_ID: String = {
            let id = Uuid::new_v4().to_string();
            println!("ID for this test is: {}", id);

            return id;
        };
    }

    pub(crate) fn new_file_connection(suffix: &str) -> Connection {
        let folder = "./integration-test-files";

        fs::create_dir_all(folder).unwrap();

        let database_path = format!("{}/{}-{}", folder, TEST_ID.to_string(), suffix);

        let connection = Connection::open(database_path, Config {
            ..Default::default()
        }).unwrap();

        return connection;
    }
}

#[cfg(feature = "integration")]
#[test]
fn test_different_value_length() {
    let mut connection = helpers::new_file_connection(stringify!(test_different_value_length));

    let mut value: Vec<u8> = vec![];
    for i in 0..1024_u32 {
        value.push((i as u8) % 255);
        connection.insert(i * 2, &value).unwrap();
        connection.check_tree_contract();
    }

    assert!(true);
}