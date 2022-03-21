use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, StdError, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

use crate::msg::Card;

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deck {
  pub cards: Vec<Card>,
}

pub fn config<S: Storage>(storage: &mut S, str: String) -> Singleton<S, Deck> {
  singleton(storage, str.as_bytes())
}

pub fn config_all_users<S: Storage>(storage: &mut S) -> Singleton<S, Vec<HumanAddr>> {
  singleton(storage, "all_users".to_string().as_bytes())
}

pub fn save_for_user<S: Storage>(
  storage: &mut S,
  user: HumanAddr,
  cards: Vec<Card>,
) -> Result<bool, StdError> {
  let mut all_users = config_all_users(storage).load().or_else(|_| Ok(vec![]))?;
  let state = Deck { cards };
  config(storage, user.to_string()).save(&state)?;
  if !all_users.contains(&user) {
    all_users.push(user.clone());
    config_all_users(storage).save(&all_users)?;
  }
  Ok(true)
}

pub fn config_read<S: Storage>(storage: &S, str: String) -> ReadonlySingleton<S, Deck> {
  singleton_read(storage, str.as_bytes())
}

pub fn config_all_users_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, Vec<HumanAddr>> {
  singleton_read(storage, "all_users".to_string().as_bytes())
}
