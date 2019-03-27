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

// comment type and entry format

pub const COMMENT_ENTRY_TYPE: &str = "comment";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CommentData {
    pub base: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Comment {
    base: String,
    author: Address,
    content: String,
}

/// Converts an input comment (without author) into a comment entry for saving to the DHT
impl CommentData {
    pub fn with_author(&self, author: Address) -> Comment {
        Comment{
            base: self.base.clone(),
            content: self.content.clone(),
            author,
        }
    }
}

// record type for base entries

pub const BASE_ENTRY_TYPE: &str = "base";

pub type Base = String;

// tag for links from base to comment

pub const COMMENT_LINK_TAG: &str = "commented_on";
