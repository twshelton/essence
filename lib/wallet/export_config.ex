
defmodule Wallet.ExportConfig do
    #Wallet credentials json 
    #    { 
    #    "key": string, Passphrase used to derive wallet master key 
    #    "path": optional path Path of the file that contains exported wallet content
    #    "key_derivation_method": optional algorithm to use for master key derivation: ARAGON2I_MOD (used by default) ARAGON2I_INT - less secured but faster 
    #    }
    @enforce_keys [:key]
    @derive {Jason.Encoder, only: [:key, :path, :key_derivation_method]}
    defstruct [:key, :path, key_derivation_method: "ARAGON2I_MOD"]
    def encode!(%Wallet.ExportConfig{}=config), do: Jason.encode!(config)
    def decode!(%Wallet.ExportConfig{}=config), do: Jason.decode!(config)
end
