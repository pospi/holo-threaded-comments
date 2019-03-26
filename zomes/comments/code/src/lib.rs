#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod comment_entry;

use hdk::{
    AGENT_ADDRESS,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
};

use comment_entry::{
    COMMENT_ENTRY_TYPE,
    CommentData,
    Comment,
    comment_from_input,
};

pub fn handle_create_comment(entry: CommentData) -> ZomeApiResult<Address> {
    // create and store the comment
    let entry = Entry::App(COMMENT_ENTRY_TYPE.into(), comment_from_input(
        entry,
        AGENT_ADDRESS.to_string().into()
    ).into());
    let address = hdk::commit_entry(&entry)?;

    // return address
    Ok(address)
}

pub fn handle_get_comment(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: COMMENT_ENTRY_TYPE,
        description: "A comment made against some other resource from elsewhere",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Comment>| {
            Ok(())
        }
    )
}

define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_comment: {
            inputs: |comment: CommentData|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_comment
        }
        get_comment: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_comment
        }
    ]

    traits: {
        hc_public [
            create_comment,
            get_comment
        ]
    }
}
