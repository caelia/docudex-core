use super::config::Config;

#[derive(Debug)]
pub struct Library {
    cfg: Config,
}

impl Library {
    fn init(cfg: Config) -> Library {
        Library { cfg }
    }
}
