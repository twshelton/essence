  defmodule NonSecrets do
        def add_wallet_record(wallet_handle,type_,id,value,tags_json), do: Indy.add_wallet_record(wallet_handle,type_,id,value,tags_json)
  def update_wallet_record_value(wallet_handle,type_,id,value), do: Indy.update_wallet_record_value(wallet_handle,type_,id,value)
  def update_wallet_record_tags(wallet_handle,type_,id,tags_json), do: Indy.update_wallet_record_tags(wallet_handle,type_,id,tags_json)
  def add_wallet_record_tags(wallet_handle,type_,id,tags_json), do: Indy.add_wallet_record_tags(wallet_handle,type_,id,tags_json)
  def delete_wallet_record_tags(wallet_handle,type_,id,tag_names_json), do: Indy.delete_wallet_record_tags(wallet_handle,type_,id,tag_names_json)
  def delete_wallet_record(wallet_handle,type_,id), do: Indy.delete_wallet_record(wallet_handle,type_,id)
  def get_wallet_record(wallet_handle,type_,id,options_json), do: Indy.get_wallet_record(wallet_handle,type_,id,options_json)
  def open_wallet_search(wallet_handle,type_,query_json,options_json), do: Indy.open_wallet_search(wallet_handle,type_,query_json,options_json)
  def fetch_wallet_search_next_records(wallet_handle,wallet_search_handle,count), do: Indy.fetch_wallet_search_next_records(wallet_handle,wallet_search_handle,count)
  def close_wallet_search(wallet_search_handle), do: Indy.close_wallet_search(wallet_search_handle)

  end
