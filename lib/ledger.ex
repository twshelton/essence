defmodule Ledger
    def sign_and_submit_request(command_handle, pool_handle,wallet_handle,submitter_did,request_json, cb), do: Indy.sign_and_submit_request(command_handle, pool_handle,wallet_handle,submitter_did,request_json, cb)
    def submit_request(command_handle, pool_handle,request_json, cb), do: Indy.submit_request(command_handle, pool_handle,request_json, cb)
    def submit_action(command_handle, pool_handle,request_json,nodes,timeout, cb), do: Indy.submit_action(command_handle, pool_handle,request_json,nodes,timeout, cb)
    def sign_request(command_handle, wallet_handle,submitter_did,request_json, cb), do: Indy.sign_request(command_handle, wallet_handle,submitter_did,request_json, cb)
    def multi_sign_request(command_handle, wallet_handle,submitter_did,request_json, cb), do: Indy.multi_sign_request(command_handle, wallet_handle,submitter_did,request_json, cb)
    def build_get_ddo_request(command_handle, submitter_did,target_did, cb), do: Indy.build_get_ddo_request(command_handle, submitter_did,target_did, cb)
    def build_nym_request(command_handle, submitter_did,target_did,verkey,alias,role, cb), do: Indy.build_nym_request(command_handle, submitter_did,target_did,verkey,alias,role, cb)
    def build_get_nym_request(command_handle, submitter_did,target_did, cb), do: Indy.build_get_nym_request(command_handle, submitter_did,target_did, cb)
    def build_attrib_request(command_handle, submitter_did,target_did,hash,raw,enc, cb), do: Indy.build_attrib_request(command_handle, submitter_did,target_did,hash,raw,enc, cb)
    def build_get_attrib_request(command_handle, submitter_did,target_did,raw,hash,enc, cb), do: Indy.build_get_attrib_request(command_handle, submitter_did,target_did,raw,hash,enc, cb)
    def build_schema_request(command_handle, submitter_did,data, cb), do: Indy.build_schema_request(command_handle, submitter_did,data, cb)
    def build_get_schema_request(command_handle, submitter_did,id, cb), do: Indy.build_get_schema_request(command_handle, submitter_did,id, cb)
    def parse_get_schema_response(command_handle, get_schema_response, cb), do: Indy.parse_get_schema_response(command_handle, get_schema_response, cb)
    def build_cred_def_request(command_handle, submitter_did,data, cb), do: Indy.build_cred_def_request(command_handle, submitter_did,data, cb)
    def build_get_cred_def_request(command_handle, submitter_did,id, cb), do: Indy.build_get_cred_def_request(command_handle, submitter_did,id, cb)
    def parse_get_cred_def_response(command_handle, get_cred_def_response, cb), do: Indy.parse_get_cred_def_response(command_handle, get_cred_def_response, cb)
    def build_node_request(command_handle, submitter_did,target_did,data, cb), do: Indy.build_node_request(command_handle, submitter_did,target_did,data, cb)
    def build_get_validator_info_request(command_handle, submitter_did, cb), do: Indy.build_get_validator_info_request(command_handle, submitter_did, cb)
    def build_get_txn_request(command_handle, submitter_did,ledger_type,seq_no, cb), do: Indy.build_get_txn_request(command_handle, submitter_did,ledger_type,seq_no, cb)
    def build_pool_config_request(command_handle, submitter_did,writes,force, cb), do: Indy.build_pool_config_request(command_handle, submitter_did,writes,force, cb)
    def build_pool_restart_request(command_handle, submitter_did,action,datetime, cb), do: Indy.build_pool_restart_request(command_handle, submitter_did,action,datetime, cb)
    def build_pool_upgrade_request(command_handle, submitter_did,name,version,action,sha256,timeout,schedule,justification,reinstall,force,package, cb), do: Indy.build_pool_upgrade_request(command_handle, submitter_did,name,version,action,sha256,timeout,schedule,justification,reinstall,force,package, cb)
    def build_revoc_reg_def_request(command_handle, submitter_did,data, cb), do: Indy.build_revoc_reg_def_request(command_handle, submitter_did,data, cb)
    def build_get_revoc_reg_def_request(command_handle, submitter_did,id, cb), do: Indy.build_get_revoc_reg_def_request(command_handle, submitter_did,id, cb)
    def parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response, cb), do: Indy.parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response, cb)
    def build_revoc_reg_entry_request(command_handle, submitter_did,revoc_reg_def_id,rev_def_type,value, cb), do: Indy.build_revoc_reg_entry_request(command_handle, submitter_did,revoc_reg_def_id,rev_def_type,value, cb)
    def build_get_revoc_reg_request(command_handle, submitter_did,revoc_reg_def_id,timestamp, cb), do: Indy.build_get_revoc_reg_request(command_handle, submitter_did,revoc_reg_def_id,timestamp, cb)
    def parse_get_revoc_reg_response(command_handle, get_revoc_reg_response, cb), do: Indy.parse_get_revoc_reg_response(command_handle, get_revoc_reg_response, cb)
    def build_get_revoc_reg_delta_request(command_handle, submitter_did,revoc_reg_def_id,from,to, cb), do: Indy.build_get_revoc_reg_delta_request(command_handle, submitter_did,revoc_reg_def_id,from,to, cb)
    def parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response, cb), do: Indy.parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response, cb)
end