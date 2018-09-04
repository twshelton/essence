defmodule Indy do
    use Rustler, otp_app: :essence

    def list_pools, do: :erlang.nif_error(:nif_not_loaded)
end
