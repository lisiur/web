#![allow(non_snake_case)]

use once_cell::sync::OnceCell;
use config::Config;

static CONF_CELL: OnceCell<Config> = OnceCell::new();

pub fn init() {
    let mut conf = Config::default();
    conf.merge(config::File::with_name("conf"))
        .unwrap()
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();
    CONF_CELL.set(conf).unwrap();
}

pub mod LOG {
    use super::CONF_CELL;

    pub fn LEVEL() -> String {
        CONF_CELL.get().unwrap().get_str("log_level").unwrap()
    }
}

pub mod DB {
    use super::CONF_CELL;

    pub fn USER() -> String {
        CONF_CELL.get().unwrap().get_str("db_user").unwrap()
    }

    pub fn PASS() -> String {
        CONF_CELL.get().unwrap().get_str("db_password").unwrap()
    }

    pub fn HOST() -> String {
        CONF_CELL.get().unwrap().get_str("db_host").unwrap()
    }

    pub fn PORT() -> String {
        CONF_CELL.get().unwrap().get_str("db_port").unwrap()
    }

    pub fn NAME() -> String {
        CONF_CELL.get().unwrap().get_str("db_name").unwrap()
    }
}

pub mod SERVICE {
    use super::CONF_CELL;

    pub fn HOST() -> String {
        CONF_CELL.get().unwrap().get_str("service_host").unwrap()
    }

    pub fn PORT() -> u16 {
        CONF_CELL.get().unwrap().get::<u16>("service_port").unwrap()
    }
}

