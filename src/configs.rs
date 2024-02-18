use serde::{Serialize, Deserialize};
use crate::{plugins::Plugins, accounts::Accounts};

#[derive(Serialize, Deserialize)]
pub struct Configs{
    pub network_name: String,
    pub accounts: Accounts,
    pub active_plugins: Plugins

}

