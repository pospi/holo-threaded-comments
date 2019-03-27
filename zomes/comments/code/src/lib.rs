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
    BASE_ENTRY_TYPE,
    COMMENT_LINK_TAG,
    Base,
};

pub fn handle_create_comment(input_entry: CommentData) -> ZomeApiResult<Address> {
    // create and store the comment
    let entry = Entry::App(COMMENT_ENTRY_TYPE.into(), comment_from_input(
        input_entry.clone(),
        AGENT_ADDRESS.to_string().into()
    ).into());
    let address = hdk::commit_entry(&entry)?;

    // store an entry for the ID of the base object the comment was made on
    let base_entry = Entry::App(BASE_ENTRY_TYPE.into(), input_entry.base.into());
    let base_address = hdk::commit_entry(&base_entry)?;

    // link the comment to its originating thing
    hdk::link_entries(
        &base_address,
        &address,
        COMMENT_LINK_TAG,
    )?;

    // return address
    Ok(address)
}

pub fn handle_get_comment(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

type ListOfEntries = Vec<ZomeApiResult<Option<Entry>>>;

pub fn handle_get_children(base: String) -> ZomeApiResult<ListOfEntries> {
    let address = hdk::entry_address(&Entry::App(BASE_ENTRY_TYPE.into(), base.into()))?;
    let links_result = hdk::get_links(&address, COMMENT_LINK_TAG);

    links_result.map(|result| {
        result.addresses().iter().map(|address| {
            hdk::get_entry(&address)
        })
        .collect()
    })
}

fn comment_def() -> ValidatingEntryType {
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

fn base_def() -> ValidatingEntryType {
    entry!(
        name: BASE_ENTRY_TYPE,
        description: "Universally unique ID of something that is being commented on",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Base>| {
            Ok(())
        },
        links: [
            to!(
                COMMENT_ENTRY_TYPE,
                tag: COMMENT_LINK_TAG,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

define_zome! {
    entries: [
       comment_def(),
       base_def()
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
        get_child_comments: {
            inputs: |address: String|,
            outputs: |result: ZomeApiResult<ListOfEntries>|,
            handler: handle_get_children
        }
    ]

    traits: {
        hc_public [
            create_comment,
            get_comment,
            get_child_comments
        ]
    }
}
