defmodule Indy.WalletTest do
  use ExUnit.Case

  setup_all do
    :ok
  end

  test "create wallet with config and credentials" do
    {return, _} = Wallet.delete(%Wallet.Config{id: "2"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {return, _} = Wallet.create(%Wallet.Config{id: "2"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    assert return == :ok
  end
  test "create/open/close  wallet with config and credentials" do
    {return, _} = Wallet.delete(%Wallet.Config{id: "12"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {return, _} = Wallet.create(%Wallet.Config{id: "12"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {:ok, handle} = Wallet.open(%Wallet.Config{id: "12"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {return, _} = Wallet.close(handle)
    assert return == :ok
  end
  test "attempt to open non-existent wallet with config and credentials" do
    {return, _} = Wallet.open(%Wallet.Config{id: "13"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    assert return == :error
  end
  test "attempt to export wallet" do
    {return, _} = Wallet.delete(%Wallet.Config{id: "14"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {return, _} = Wallet.create(%Wallet.Config{id: "14"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {:ok, handle} = Wallet.open(%Wallet.Config{id: "14"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    File.rm("/tmp/exported_wallet")
    {return, _} = Wallet.export(handle, %Wallet.ExportConfig{key: "thasedrvj44oierglkerv'ijrijrgjerb", path: "/tmp/exported_wallet"})
    assert return == :ok
  end
  test "attempt to import wallet" do
    {return, _} = Wallet.delete(%Wallet.Config{id: "15"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {return, _} = Wallet.create(%Wallet.Config{id: "15"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    {:ok, handle} = Wallet.open(%Wallet.Config{id: "15"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"})
    File.rm("/tmp/exported_wallet1")
    {return, _} = Wallet.export(handle, %Wallet.ExportConfig{key: "thasedrvj44oierglkerv'ijrijrgjerb", path:
    "/tmp/exported_wallet1"})
    {return, _} = Wallet.import(%Wallet.Config{id: "16"}, %Wallet.Credentials{key: "thasedrvj44oierglkerv'ijrijrgjerb"},%Wallet.ImportConfig{key: "thasedrvj44oierglkerv'ijrijrgjerb", path: "/tmp/exported_wallet1"})
    assert return == :ok
  end
end
