#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;

extern crate libc;
extern crate indy;

mod utils;
mod api;

use api::{pool, wallet, did};

rustler_export_nifs! {
    "Elixir.Indy",
    [
        ("list_pools", 0, pool::list_pools),
        ("set_protocol_version", 1, pool::set_protocol_version),
        ("open_default_pool_ledger", 1, pool::open_default_pool_ledger),
        ("open_pool_ledger", 2, pool::open_pool_ledger),
        ("refresh_pool_ledger", 1, pool::refresh_pool_ledger),
        ("close_pool_ledger", 1, pool::close_pool_ledger),
        ("create_pool_ledger_config", 2, pool::create_pool_ledger_config),
        ("delete_pool_ledger_config", 1, pool::delete_pool_ledger_config),
        ("import_wallet", 3, wallet::import_wallet),
        ("export_wallet", 2, wallet::export_wallet),
        ("delete_wallet", 2, wallet::delete_wallet),
        ("close_wallet", 1, wallet::close_wallet),
        ("open_wallet", 2, wallet::open_wallet),
        ("create_wallet", 2, wallet::create_wallet),
        ("abbreviate_verkey", 1, did::abbreviate_verkey),
        ("list_my_dids_with_meta", 1, did::list_my_dids_with_meta),
        ("get_my_did_with_meta", 2, did::get_my_did_with_meta),
        ("set_did_metadata", 3, did::set_did_metadata),
        ("replace_keys_apply", 2, did::replace_keys_apply),
        ("replace_keys_start", 3, did::replace_keys_start),
        ("create_and_store_my_did", 2, did::create_and_store_my_did),
        ("store_their_did",2,did::store_their_did),
        ("key_for_did",3,did::key_for_did),
        ("key_for_local_did",2,did::key_for_local_did),
        ("set_endpoint_for_did",4,did::set_endpoint_for_did),
        ("get_endpoint_for_did",3,did::get_endpoint_for_did),
        ("get_did_metadata",2,did::get_did_metadata),
        ("sign_and_submit_request",4,ledger::sign_and_submit_request), 
        ("submit_request",2,ledger::submit_request), 
        ("submit_action",4,ledger::submit_action), 
        ("sign_request",3,ledger::sign_request), 
        ("multi_sign_request",3,ledger::multi_sign_request), 
        ("build_get_ddo_request",2,ledger::build_get_ddo_request), 
        ("build_nym_request",5,ledger::build_nym_request), 
        ("build_get_nym_request",2,ledger::build_get_nym_request), 
        ("build_attrib_request",5,ledger::build_attrib_request), 
        ("build_get_attrib_request",5,ledger::build_get_attrib_request), 
        ("build_schema_request",2,ledger::build_schema_request), 
        ("build_get_schema_request",2,ledger::build_get_schema_request), 
        ("parse_get_schema_response",1,ledger::parse_get_schema_response), 
        ("build_cred_def_request",2,ledger::build_cred_def_request), 
        ("build_get_cred_def_request",2,ledger::build_get_cred_def_request), 
        ("parse_get_cred_def_response",1,ledger::parse_get_cred_def_response), 
        ("build_node_request",3,ledger::build_node_request), 
        ("build_get_validator_info_request",1,ledger::build_get_validator_info_request), 
        ("build_get_txn_request",3,ledger::build_get_txn_request), 
        ("build_pool_config_request",3,ledger::build_pool_config_request), 
        ("build_pool_restart_request",3,ledger::build_pool_restart_request), 
        ("build_pool_upgrade_request",11,ledger::build_pool_upgrade_request), 
        ("build_revoc_reg_def_request",2,ledger::build_revoc_reg_def_request), 
        ("build_get_revoc_reg_def_request",2,ledger::build_get_revoc_reg_def_request), 
        ("parse_get_revoc_reg_def_response",1,ledger::parse_get_revoc_reg_def_response), 
        ("build_revoc_reg_entry_request",4,ledger::build_revoc_reg_entry_request), 
        ("build_get_revoc_reg_request",3,ledger::build_get_revoc_reg_request), 
        ("parse_get_revoc_reg_response",1,ledger::parse_get_revoc_reg_response), 
        ("build_get_revoc_reg_delta_request",4,ledger::build_get_revoc_reg_delta_request), 
        ("parse_get_revoc_reg_delta_response",1,ledger::parse_get_revoc_reg_delta_response)
    ],
    None
}

