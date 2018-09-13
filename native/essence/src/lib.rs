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
    ],
    None
}

