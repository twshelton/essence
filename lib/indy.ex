defmodule Indy do
    use Rustler, otp_app: :essence, crate: "essence"

    #Pool definitions
    def delete_pool_ledger_config(config_name), do: nif_error() 
    def open_pool_ledger(config_name, config), do: nif_error() 
    def refresh_pool_ledger(handle), do: nif_error() 
    def close_pool_ledger(handle), do: nif_error() 
    def open_default_pool_ledger(config_name), do: nif_error() 
    def create_pool_ledger_config(config_name, config), do: nif_error() 
    def set_protocol_version(version), do: nif_error() 
    def list_pools(), do: nif_error() 

    #Wallet definitions
    def create_wallet(config, credentials), do: nif_error() 
    def import_wallet(config, credentials, import_config), do: nif_error() 
    def export_wallet(handle, export_config), do: nif_error() 
    def delete_wallet(config, credentials), do: nif_error() 
    def close_wallet(handle), do: nif_error() 
    def open_wallet(config, credentials), do: nif_error()

    #Did definitions
    def create_and_store_my_did(wallet_handle, did_json), do: nif_error() 
    def replace_keys_start(wallet_handle, did, key_info), do: nif_error() 
    def replace_keys_apply(wallet_handle, did), do: nif_error() 
    def store_their_did(wallet_handle, identity_json), do: nif_error()
    def key_for_did(pool_handle, wallet_handle, did), do: nif_error()
    def key_for_local_did(wallet_handle, did), do: nif_error()
    def set_endpoint_for_did(wallet_handle, did, address, transport_key), do: nif_error()
    def get_endpoint_for_did(wallet_handle, pool_handle, did), do: nif_error()
    def set_did_metadata(wallet_handle, did, metdata), do: nif_error() 
    def get_did_metadata(wallet_handle, did), do: nif_error() 
    def get_my_did_with_meta(wallet_handle, did), do: nif_error() 
    def list_my_dids_with_meta(wallet_handle), do: nif_error() 
    def abbreviate_verkey(handle), do: nif_error() 

    def nif_error, do: :erlang.nif_error(:nif_not_loaded)
end
