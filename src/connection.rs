use crate::btree::operations::insert::do_insert as do_operation_insert;
use crate::pager::Config as PagerConfig;
use crate::pager::Pager;
use crate::btree::operations as BTreeOperations;

#[derive(Debug)]
pub struct Config {
    pub cache_size_mb: u32,
    pub create: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cache_size_mb: 100,
            create: true,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Connection {
    config: Config,
    pub(crate) pager: Pager,
}

impl Connection {
    /// Connection is the main entrypoint for the library, which means that all
    /// initializations will be performed in this method
    pub fn open(main_db_path: String, config: Config) -> std::io::Result<Connection> {
        let pager_config = PagerConfig { main_db_path };

        let mut pager = Pager::new(pager_config)?;

        BTreeOperations::initialize::root_node_if_needed(&mut pager);

        Ok(Connection {
            config,
            pager,
        })
    }

    pub fn insert(&mut self, key: u32, value: &[u8]) -> Result<(), std::io::Error> {
        do_operation_insert(&mut self.pager, key, value)
    }

    #[cfg(feature = "integration")]
    pub fn check_tree_contract(&self) {
        use crate::btree::operations::check_contract::do_check_tree_contract;

        do_check_tree_contract(&self.pager)
    }
}