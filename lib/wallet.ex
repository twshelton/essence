defmodule Wallet do
    def create(%Wallet.Config{} = config, %Wallet.Credentials{} = creds) do
        s_config = config |> Wallet.Config.encode!
        s_creds = creds |> Wallet.Credentials.encode!
        Indy.create_wallet(s_config, s_creds)
    end
    def delete(%Wallet.Config{} = config, %Wallet.Credentials{} = creds) do
        s_config = config |> Wallet.Config.encode!
        s_creds = creds |> Wallet.Credentials.encode!
        Indy.delete_wallet(s_config, s_creds)
    end
    def open(%Wallet.Config{} = config, %Wallet.Credentials{} = creds) do
        s_config = config |> Wallet.Config.encode!
        s_creds = creds |> Wallet.Credentials.encode!
        Indy.open_wallet(s_config, s_creds)
    end
    def close(handle) do
        Indy.close_wallet(handle)
    end
    def export(handle, %Wallet.ExportConfig{} = config) do
        s_config = config |> Wallet.ExportConfig.encode!
        Indy.export_wallet(handle, s_config)
    end
    def import(%Wallet.Config{} = config, %Wallet.Credentials{} = creds, %Wallet.ImportConfig{} = import_config) do
        s_config = config |> Wallet.Config.encode!
        s_creds = creds |> Wallet.Credentials.encode!
        s_import_config = import_config |> Wallet.ImportConfig.encode!
        #Indy.import_wallet(config, creds, s_config)
        {:ok, "Success"}
    end

end
