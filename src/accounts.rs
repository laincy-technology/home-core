use std::{env, fs::read_dir, mem::variant_count};
use color_eyre::eyre::Result;
use serde::{Serialize, Deserialize};



pub type Accounts = Vec<Account>;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub uuid: u16,
    pub username: String,
    pub password: String,
    pub permissions: Permissions
}

#[derive(Deserialize)]
struct TempAccount {
    pub username: String,
    pub password: String,
    pub permissions: Permissions
}

#[derive(Serialize, Deserialize)]
pub enum Permission {
    EditConnections(bool),
    EditPlugins(bool),
    EditUsers(bool)
}

pub type Permissions = [Permission; variant_count::<Permission>()];

pub async fn get() -> Result<Accounts> {
    let mut accounts = Accounts::new();


    
    Ok(accounts)
}

impl Account {

    pub fn has_perm(&self, perm: Permission) -> bool {
        unimplemented!()
    }

}
