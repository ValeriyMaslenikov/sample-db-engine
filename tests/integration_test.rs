use std::{ fs::File, io::Read, os::unix::prelude::FileExt, mem::size_of };

use bincode::{
    decode_from_slice,
    config::{ Configuration, LittleEndian, Fixint, SkipFixedArrayLength },
};
use engine::PAGE_SIZE;

/// The purpose of these tests is to do tree validation after the execution of multiple operations.

type DatabasePath = String;

const BINCODE_CONFIG: Configuration<LittleEndian, Fixint, SkipFixedArrayLength> = bincode::config
    ::standard()
    .with_fixed_int_encoding()
    .skip_fixed_array_length();
    
#[cfg(feature = "integration")]
mod helpers {
    use std::fs;

    use engine::connection::Connection;
    use lazy_static::lazy_static;
    use uuid::Uuid;

    use crate::DatabasePath;

    lazy_static! {
        static ref TEST_ID: String = {
            let id = Uuid::new_v4().to_string();
            println!("ID for this test is: {}", id);

            return id;
        };
    }

    pub(crate) fn new_file_connection(suffix: &str) -> (Connection, DatabasePath) {
        let folder = "./integration-test-files";

        fs::create_dir_all(folder).unwrap();

        let database_path = format!("{}/{}-{}", folder, TEST_ID.to_string(), suffix);

        let connection = Connection::open(database_path.clone(), engine::connection::Config {
            ..Default::default()
        }).unwrap();

        return (connection, database_path);
    }
}

#[cfg(feature = "integration")]
#[test]
fn test_different_value_length() {
    let (mut connection, _) = helpers::new_file_connection(
        stringify!(test_different_value_length)
    );

    let mut value: Vec<u8> = vec![];
    for i in 0..1024_u32 {
        value.push((i as u8) % 255);
        connection.insert(i * 2, &value).unwrap();
        connection.check_tree_contract();
    }

    assert!(true);
}

#[test]
fn test_structure_of_leaf_will_correspond_described_in_article() {
    // This test covers the example provided in the article #2
    // where the structure of the node is visualized
    let (mut connection, database_path) = helpers::new_file_connection(
        stringify!(test_structure_of_leaf_will_correspond_described_in_article)
    );

    let word_ten_as_bytes = "Ten".as_bytes();
    connection.insert(10, word_ten_as_bytes).unwrap();
    let word_three_as_bytes = "Three".as_bytes();
    connection.insert(3, word_three_as_bytes).unwrap();

    drop(connection);

    let db_file = File::open(database_path).unwrap();

    let mut buffer = [0_u8; PAGE_SIZE as usize];

    db_file.read_exact_at(&mut buffer, 0).unwrap();

    let node_buffer = &buffer[100..];

    let (decoded, decoded_bytes): (
        (
            [u8; 100],
            // node_type
            u32,
            // free_space_start_offset
            u32,
            // free_space_end_offset
            u32,
            // elements_count
            u32,
        ),
        usize,
    ) = decode_from_slice(&buffer, BINCODE_CONFIG).unwrap();

    assert_eq!(decoded_bytes, 100 + 16);
    assert_eq!(decoded.0.len(), 100);
    assert_eq!(decoded.1, 1);
    assert_eq!(decoded.2, 40);
    // In the medium article we use value 4088 as the reference. But in our case we can't
    // reproduce the same situation. Here it's the first page and the whole size of the node is not
    // 4096, it's 4096 - 100 (database header size).
    assert_eq!(decoded.3, 4088 - 100);

    // Checking free space, in article it's mentioned as 4016. 
    // 100 - db header, 16 - node header, 2x 12 - node header, 3 – word "ten", 5 - word "three"
    assert_eq!(decoded.3 - decoded.2, PAGE_SIZE - 100 - 16 -  12 - 12 - 3 - 5);
    // The same as previous, to validate that in article specified the same value as produced by 
    // the program
    assert_eq!(decoded.3 - decoded.2, 4048 - 100);

    assert_eq!(decoded.4, 2);



    // Verify the key reference
    let (key_ref_decoded, decoded_size): (
        (u32, u32, u32, u32, u32, u32),
        usize,
    ) = decode_from_slice(&node_buffer[16..], BINCODE_CONFIG).unwrap();

    assert_eq!(decoded_size, size_of::<u32>() * 6);

    let word_ten_offset = node_buffer.len() - word_ten_as_bytes.len();
    assert_eq!(word_ten_offset, 4093 - 100);
    let word_three_offset = word_ten_offset - word_three_as_bytes.len();
    assert_eq!(word_three_offset, 4088 - 100);

    assert_eq!(key_ref_decoded.0, 3);
    assert_eq!(key_ref_decoded.1 as usize, word_three_as_bytes.len());
    assert_eq!(key_ref_decoded.2 as usize, word_three_offset);

    assert_eq!(key_ref_decoded.3, 10);
    assert_eq!(key_ref_decoded.4 as usize, word_ten_as_bytes.len());
    assert_eq!(key_ref_decoded.5 as usize, word_ten_offset);


    // Verify data part
    // First one was added "Three", it should be in the end
    assert_eq!(node_buffer[word_ten_offset..], *word_ten_as_bytes);
    assert_eq!(node_buffer[word_three_offset..word_ten_offset], *word_three_as_bytes);
}