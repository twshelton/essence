#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;

extern crate libc;
extern crate indy;

mod callbacks;
mod utils;
mod results;
mod did;
mod pool;

rustler_export_nifs! {
    "Elixir.Indy",
    [
        ("Did.create_did", 0, did::create_did),
        ("list_pools", 0, list_pools),
        ("set_protocol_version", 1, set_protocol_version),
        ("open_default_pool_ledger", 1, open_default_pool_ledger),
        ("open_pool_ledger", 2, open_pool_ledger),
        ("refresh_pool_ledger", 1, refresh_pool_ledger),
        ("close_pool_ledger", 1, close_pool_ledger),
        ("create_pool_ledger_config", 2, create_pool_ledger_config),
        ("delete_pool_ledger_config", 1, delete_pool_ledger_config)
    ],
    None
}

