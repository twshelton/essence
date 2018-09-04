defmodule Indy.Pool do
    use Rustler, otp_app: :essence, crate: "pool"

    # When your NIF is loaded, it will override this function.
    def delete_pool_ledger_config(config_name), do: :erlang.nif_error(:nif_not_loaded)
    def open_pool_ledger(config_name, config), do: :erlang.nif_error(:nif_not_loaded)
    def refresh_pool_ledger(handle), do: :erlang.nif_error(:nif_not_loaded)
    def close_pool_ledger(handle), do: :erlang.nif_error(:nif_not_loaded)
    def open_default_pool_ledger(config_name), do: :erlang.nif_error(:nif_not_loaded)
    def create_pool_ledger_config(config_name, config), do: :erlang.nif_error(:nif_not_loaded)
    def set_protocol_version(version), do: :erlang.nif_error(:nif_not_loaded)
    def list_pools(), do: :erlang.nif_error(:nif_not_loaded)
end
