use crate::config::app_config::AppConfig;

#[derive(Debug, Clone)]
pub struct ConfigService {
    pub config: AppConfig,
}

impl ConfigService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn get_config(&self) -> AppConfig {
        self.config.clone()
    }
}
