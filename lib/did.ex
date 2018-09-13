defmodule Did do
    def create_and_store_my_did(wallet_handle, did_json), do: Indy.create_and_store_my_did(wallet_handle, did_json)
    def replace_keys_start(wallet_handle, did, key_info), do: Indy.replace_keys_start(wallet_handle, did, key_info)
    def replace_keys_apply(wallet_handle, did), do: Indy.replace_keys_apply(wallet_handle, did)
    def set_did_metadata(wallet_handle, did, metdata), do: Indy.set_did_metadata(wallet_handle, did, metdata)
    def get_my_did_with_meta(wallet_handle, did), do: Indy.get_my_did_with_meta(wallet_handle, did)
    def list_my_dids_with_meta(wallet_handle), do: Indy.list_my_dids_with_meta(wallet_handle)
    def abbreviate_verkey(handle), do: Indy.abbreviate_verkey(handle)
    def store_their_did(wallet_handle, identity_json), do: Indy.store_their_did(wallet_handle, identity_json)
    def key_for_did(pool_handle, wallet_handle, did), do: Indy.key_for_did(pool_handle, wallet_handle, did)
    def key_for_local_did(wallet_handle, did), do: Indy.key_for_local_did(wallet_handle, did)
    def set_endpoint_for_did(wallet_handle, did, address, transport_key), do: Indy.set_endpoint_for_did(wallet_handle, did, address, transport_key)
    def get_endpoint_for_did(wallet_handle, pool_handle, did), do: Indy.get_endpoint_for_did(wallet_handle, pool_handle, did)
    def get_did_metadata(wallet_handle, did), do: Indy.get_did_metadata(wallet_handle, did)
end

