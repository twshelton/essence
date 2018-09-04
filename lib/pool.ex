defmodule Pool do
    def has_ledger_config?(config) do
        list
        |> Enum.count(fn(%{"pool" => v}) -> v == config end)
        |> case do
            0 -> false
            _ -> true
        end
    end
    
    def refresh_ledger(handle), do: Indy.Pool.refresh_pool_ledger(handle)
    def close_ledger(handle), do: Indy.Pool.close_pool_ledger(handle)
    def open_ledger(name), do: Indy.Pool.open_default_pool_ledger(name)
    def open_ledger(name, config), do: Indy.Pool.open_pool_ledger(name, config)

    def create_ledger_config(name), do: create_ledger_config(name, Application.get_env(:essence, :genesis_transaction_file))
    def create_ledger_config(name, config), do: Indy.Pool.create_pool_ledger_config(name, config)

    def delete_ledger_config(config) do
        case has_ledger_config?(config) do
            true ->
                Indy.Pool.delete_pool_ledger_config(config)
                :ok
            _ ->
                :ok
        end
    end
    def list do
        case Indy.Pool.list_pools do
            {:ok, json} ->
                json |> Jason.decode!
            {_, _} ->
                []
        end
    end
end
