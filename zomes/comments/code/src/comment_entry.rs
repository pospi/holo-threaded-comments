/**
 * Comment entry type definition
 *
 * @package: Holochain comments
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-03-26
 */

use holochain_core_types_derive::{ DefaultJson };

use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
};

pub const COMMENT_ENTRY_TYPE: &str = "comment";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Comment {
    base: Address,
    content: String,
}
