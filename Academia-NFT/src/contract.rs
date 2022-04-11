#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg,MintMsg,ModuleProgressMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,Order};
use cw721::{
    ContractInfoResponse};

use crate::state::{increment_tokens,TokenInfo,tokens,MAP_PROGRESS,ModuleProgress,NFT_PROGRESS,VIDEOS_WATCHED,TOKEN_COUNT};



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let _info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint(msg) => execute_mint(deps, env, info, msg),
        ExecuteMsg::UpdateProgress(msg) => update_progress(deps, env, info, msg)
    }
}

pub fn update_progress(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ModuleProgressMsg,
)
-> Result<Response, ContractError> 
{
    let progress=ModuleProgress{
        module: msg.module,
        video_completed: msg.video_completed ,
        date_completed: msg.date_completed  
    };
    let module=msg.module.to_string();
    let sender_key=info.sender.to_string()+"_"+&module;
    let  videos_watched =match VIDEOS_WATCHED.may_load(deps.storage, &sender_key)?
    {Some(data)=>Some(data),
     None=>Some(vec![])};
    let mut updated_list=videos_watched.unwrap();
    updated_list.push(progress.video_completed.to_string());
    MAP_PROGRESS.save(deps.storage, &sender_key,&progress)?;
    VIDEOS_WATCHED.save(deps.storage, &sender_key, &updated_list)?;
    Ok(Response::new()
    .add_attribute("Sender_key", sender_key)
    .add_attribute("Module", module)
    .add_attribute("Video_completed", progress.video_completed.to_string())
    )
}

pub fn execute_mint(
     deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: MintMsg,
) -> Result<Response, ContractError> {
    
    let suffix="acad_nft_".to_string();
    let tokenid=TOKEN_COUNT.may_load(deps.storage)?.unwrap_or_default()+1;
    let token_id=suffix+&tokenid.to_string();
    let sender_key=info.sender.to_string()+"_"+&msg.module.to_string();
    let token = TokenInfo {
        owner: deps.api.addr_validate(&msg.owner)?,
        module_completed: msg.module_completed,
        date_completed: msg.date_completed
    };

    tokens().update(deps.storage, &token_id, |old| match old {
        Some(_) => Err(ContractError::Claimed {}),
        None => Ok(token),
    })?;
    NFT_PROGRESS.save(deps.storage, &sender_key,&token_id)?;

    increment_tokens(deps.storage)?;
    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("token_id", token_id))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::OwnerOf{token_id} => to_binary(&query_owner_of(deps, token_id)?),
        QueryMsg::NftInfo{token_id} => to_binary(&query_nft_info(deps, token_id)?),
        QueryMsg::OwnedTokenComplete{owner_id} => to_binary(&query_owner_nft_complete(deps,owner_id)?),
        QueryMsg::QueryNFT{sender_key} => to_binary(&get_nft_progress(deps,sender_key)?),
        QueryMsg::QueryProgress{sender_key}=>to_binary(&get_module_progress(deps,sender_key)?)
    }
}

fn get_module_progress(deps: Deps,sender_key: String) -> StdResult<Vec<String>> {
    let info= match VIDEOS_WATCHED.may_load(deps.storage,&sender_key )?{ 
            Some(record) => Some(record),
            None => Some(vec![]) 
    };

    Ok(info.unwrap())
}

fn get_nft_progress(deps: Deps,sender_key: String) -> StdResult<String> {
    let info =match  NFT_PROGRESS.may_load(deps.storage, &sender_key)?{
        Some(record) => Some(record),
        None => Some("None".to_string()) 
    };

    Ok(info.unwrap())
}

fn query_owner_of(deps: Deps,token_id: String) -> StdResult<String> {
    let info = tokens().load(deps.storage, &token_id)?;
    Ok(info.owner.to_string())
}

fn query_nft_info(deps: Deps,token_id: String) -> StdResult<TokenInfo> {
    let info = tokens().load(deps.storage, &token_id)?;
    Ok(info)
}

fn query_owner_nft_complete(
    deps: Deps,
    owner: String,
    
) -> StdResult<std::vec::Vec<((), TokenInfo)>> {

    let owner_addr = deps.api.addr_validate(&owner)?;
    let pks: Vec<_> = tokens()
        .idx
        .owner
        .prefix(owner_addr)
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(pks)
}