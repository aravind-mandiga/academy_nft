use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::{Map};
use cosmwasm_std::{  StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};






#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]

pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub owner: Addr,
    pub module_completed: String,
    pub date_completed: String,
     }

     #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
     pub struct ModuleProgress {
         pub module: i32,
         pub video_completed: i32,
         pub date_completed: String,
     }     

pub const MAP_PROGRESS:Map<&str,ModuleProgress>=Map::new("module progress");
pub const NFT_PROGRESS:Map<&str,String>=Map::new("NFT progress");
pub const VIDEOS_WATCHED:Map<&str,Vec<String>>=Map::new("Videos_watched");

pub const TOKEN_COUNT: Item<u64> = Item::new("num_tokens");

pub fn num_tokens(storage: &dyn Storage) -> StdResult<u64> {
    Ok(TOKEN_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_tokens(storage: &mut dyn Storage) -> StdResult<u64> {
    let val = num_tokens(storage)? + 1;
    TOKEN_COUNT.save(storage, &val)?;
    Ok(val)
}

pub fn decrement_tokens( storage: &mut dyn Storage) -> StdResult<u64> {
    let val = num_tokens(storage)? - 1;
    TOKEN_COUNT.save(storage, &val)?;
    Ok(val)
}

pub struct TokenIndexes<'a> {
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, Addr, TokenInfo>,
}

impl<'a> IndexList<TokenInfo> for TokenIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenInfo>> + '_> {
        let v: Vec<&dyn Index<TokenInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn tokens<'a>() -> IndexedMap<'a, &'a str, TokenInfo, TokenIndexes<'a>> {
    let indexes = TokenIndexes {
        owner: MultiIndex::new(
            |d: &TokenInfo|(d.owner.clone()),
            "tokens",
            "tokens__owner",
        ),
    };
    IndexedMap::new("tokens", indexes)
}




