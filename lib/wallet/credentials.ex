defmodule Wallet.Credentials do
    #Wallet credentials json 
    #    { 
    #    "key": string, Passphrase used to derive wallet master key 
    #    "storage_credentials": optional Credentials for wallet storage. Storage type defines set of
    #        supported keys. Can be optional if storage supports default configuration. For 'default' storage type should be empty.
    #    "key_derivation_method": optional algorithm to use for master key derivation: ARAGON2I_MOD (used by default) ARAGON2I_INT - less secured but faster 
    #    }
    @enforce_keys [:key]
    @derive {Jason.Encoder, only: [:key, :storage_credentials, :key_derivation_method]}
    defstruct [:key, storage_credentials: "default", key_derivation_method: "ARAGON2I_MOD"]
    def encode!(%Wallet.Credentials{}=creds), do: Jason.encode!(creds)
    def decode!(%Wallet.Credentials{}=creds), do: Jason.decode!(creds)
end
