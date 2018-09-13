defmodule Wallet.Config do
    #Wallet configuration json. 
    #    { 
    #    "id": string, Identifier of the wallet. Configured storage uses this identifier to lookup exact wallet data placement. 
    #    "storage_type": optional, Type of the wallet storage. Defaults to 'default'. 'Default' storage type allows to store 
    #        wallet data in the local file. Custom storage types can be registered with indy_register_wallet_storage call. 
    #    "storage_config": optional, Storage configuration json. Storage type defines set of supported keys. Can be optional if 
    #        storage supports default configuration. For 'default' storage type configuration is: { "path": optional, Path to the directory with wallet files. 
    #        Defaults to $HOME/.indy_client/wallets. Wallet will be stored in the file {path}/{id}/sqlite.db } } 
    @enforce_keys [:id]
    @derive {Jason.Encoder, only: [:id, :storage_type, :storage_config]}
    defstruct [:id, storage_type: "default", storage_config: %{:path => "#{:os.getenv('HOME')}/.indy_client/wallets"}]
    def encode!(%Wallet.Config{}=config), do: Jason.encode!(config)
    def decode!(%Wallet.Config{}=config), do: Jason.decode!(config)
end
