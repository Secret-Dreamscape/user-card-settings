use cosmwasm_std::{
  to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, StdError,
  StdResult, Storage,
};

use crate::msg::{AllUsersResponse, Card, HandleMsg, InitMsg, QueryMsg, SettingsResponse};
use crate::state::{config_all_users_read, config_read, save_for_user};

pub fn init<S: Storage, A: Api, Q: Querier>(
  _deps: &mut Extern<S, A, Q>,
  _env: Env,
  _msg: InitMsg,
) -> StdResult<InitResponse> {
  Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  msg: HandleMsg,
) -> StdResult<HandleResponse> {
  match msg {
    HandleMsg::Save { cards } => try_save(deps, env, cards),
  }
}

fn try_save<S: Storage, A: Api, Q: Querier>(
  deps: &mut Extern<S, A, Q>,
  env: Env,
  cards: Vec<Card>,
) -> Result<HandleResponse, StdError> {
  save_for_user(&mut deps.storage, env.message.sender, cards)?;
  Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
  deps: &Extern<S, A, Q>,
  msg: QueryMsg,
) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetSettings { user } => to_binary(&query_settings(deps, user)?),
    QueryMsg::GetAllUsers {} => to_binary(&query_all_users(deps)?),
  }
}

fn query_settings<S: Storage, A: Api, Q: Querier>(
  deps: &Extern<S, A, Q>,
  user: HumanAddr,
) -> StdResult<SettingsResponse> {
  let all_users = config_all_users_read(&deps.storage).load()?;
  // check if user is in the list of all users or throw an error
  if !all_users.contains(&user) {
    return Err(StdError::generic_err(format!(
      "No settings found for user {}",
      user
    )));
  }

  let state = config_read(&deps.storage, user.to_string()).load()?;
  Ok(SettingsResponse {
    cards: state.cards.clone(),
  })
}

fn query_all_users<S: Storage, A: Api, Q: Querier>(
  deps: &Extern<S, A, Q>,
) -> StdResult<AllUsersResponse> {
  let state = config_all_users_read(&deps.storage).load()?;
  Ok(AllUsersResponse { users: state })
}
