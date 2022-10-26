use crate::pager::Config as PagerConfig;
use crate::pager::Pager;

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
    pager: Pager,
}

impl Connection {
    /// Connection is the main entrypoint for the library, which means that all
    /// initializations will be performed in this method
    pub fn open(main_db_path: String, config: Config) -> std::io::Result<Connection> {
        let pager_config = PagerConfig { main_db_path };

        let pager = Pager::new(pager_config)?;

        Ok(Connection {
            config: config,
            pager,
        })
    }
}