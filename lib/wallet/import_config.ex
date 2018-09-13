
defmodule Wallet.ImportConfig do
    #    { 
    #    "key": string, Passphrase used to derive wallet master key 
    #    "path": optional path Path of the file that contains exported wallet content
    #    }
    @enforce_keys [:key]
    @derive {Jason.Encoder, only: [:key, :path]}
    defstruct [:key, :path]
    def encode!(%Wallet.ImportConfig{}=config), do: Jason.encode!(config)
    def decode!(%Wallet.ImportConfig{}=config), do: Jason.decode!(config)
end
