  defmodule Payments do
        def register_payment_method(payment_method,create_payment_address,add_request_fees,parse_response_with_fees,build_get_payment_sources_request,parse_get_payment_sources_response,build_payment_req,parse_payment_response,build_mint_req,build_set_txn_fees_req,build_get_txn_fees_req,parse_get_txn_fees_response,build_verify_payment_req,parse_verify_payment_response), do: Indy.register_payment_method(payment_method,create_payment_address,add_request_fees,parse_response_with_fees,build_get_payment_sources_request,parse_get_payment_sources_response,build_payment_req,parse_payment_response,build_mint_req,build_set_txn_fees_req,build_get_txn_fees_req,parse_get_txn_fees_response,build_verify_payment_req,parse_verify_payment_response)
  def create_payment_address(wallet_handle,payment_method,config), do: Indy.create_payment_address(wallet_handle,payment_method,config)
  def list_payment_addresses(wallet_handle), do: Indy.list_payment_addresses(wallet_handle)
  def add_request_fees(wallet_handle,submitter_did,req_json,inputs_json,outputs_json,extra), do: Indy.add_request_fees(wallet_handle,submitter_did,req_json,inputs_json,outputs_json,extra)
  def parse_response_with_fees(payment_method,resp_json), do: Indy.parse_response_with_fees(payment_method,resp_json)
  def build_get_payment_sources_request(wallet_handle,submitter_did,payment_address), do: Indy.build_get_payment_sources_request(wallet_handle,submitter_did,payment_address)
  def parse_get_payment_sources_response(payment_method,resp_json), do: Indy.parse_get_payment_sources_response(payment_method,resp_json)
  def build_payment_req(wallet_handle,submitter_did,inputs_json,outputs_json,extra), do: Indy.build_payment_req(wallet_handle,submitter_did,inputs_json,outputs_json,extra)
  def parse_payment_response(payment_method,resp_json), do: Indy.parse_payment_response(payment_method,resp_json)
  def build_mint_req(wallet_handle,submitter_did,outputs_json,extra), do: Indy.build_mint_req(wallet_handle,submitter_did,outputs_json,extra)
  def build_set_txn_fees_req(wallet_handle,submitter_did,payment_method,fees_json), do: Indy.build_set_txn_fees_req(wallet_handle,submitter_did,payment_method,fees_json)
  def build_get_txn_fees_req(wallet_handle,submitter_did,payment_method), do: Indy.build_get_txn_fees_req(wallet_handle,submitter_did,payment_method)
  def parse_get_txn_fees_response(payment_method,resp_json), do: Indy.parse_get_txn_fees_response(payment_method,resp_json)
  def build_verify_payment_req(wallet_handle,submitter_did,receipt), do: Indy.build_verify_payment_req(wallet_handle,submitter_did,receipt)
  def parse_verify_payment_response(payment_method,resp_json), do: Indy.parse_verify_payment_response(payment_method,resp_json)

  end
