defmodule Indy.PoolTest do
  use ExUnit.Case

  test "set protocol to correct version" do
    {return, _} = Indy.set_protocol_version(1)
    assert return == :ok
  end
  test "set protocol to incorrect version" do
    {return, _} = 
        Indy.set_protocol_version(0)
        |> IO.inspect
    assert return == :error
  end
  test "create pool ledger config with default file" do
    # this will use the file defined in the Essence Config file :genesis_transaction_file
    Pool.delete_ledger_config("random1")
    {return, _} = Pool.create_ledger_config("random1")
    assert return == :ok
  end
  test "refresh pool ledger" do
    {return, _} = Indy.set_protocol_version(2)
    {return, ihandle} = Pool.open_ledger("some_name")
    {return, _} = Pool.refresh_ledger(ihandle)
    {return, _} = Pool.close_ledger(ihandle)
    assert return == :ok
  end
end
