defmodule Pairwise do
  def is_pairwise_exists(wallet_handle,their_did), do: Indy.is_pairwise_exists(wallet_handle,their_did)
  def create_pairwise(wallet_handle,their_did,my_did,metadata), do: Indy.create_pairwise(wallet_handle,their_did,my_did,metadata)
  def list_pairwise(wallet_handle), do: Indy.list_pairwise(wallet_handle)
  def get_pairwise(wallet_handle,their_did), do: Indy.get_pairwise(wallet_handle,their_did)
  def set_pairwise_metadata(wallet_handle,their_did,metadata), do: Indy.set_pairwise_metadata(wallet_handle,their_did,metadata)
end
