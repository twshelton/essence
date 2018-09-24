use std::ffi::CString;
use rustler::{Env, Term, NifResult,Encoder};
use utils::ex_results::{result_to_string,result_to_string_string,result_to_string_string_timestamp};
use utils::callbacks;
use indy::api::ledger::{indy_sign_and_submit_request,
indy_submit_request,
indy_submit_action,
indy_sign_request,
indy_multi_sign_request,
indy_build_get_ddo_request,
indy_build_nym_request,
indy_build_get_nym_request,
indy_build_attrib_request,
indy_build_get_attrib_request,
indy_build_schema_request,
indy_build_get_schema_request,
indy_parse_get_schema_response,
indy_build_cred_def_request,
indy_build_get_cred_def_request,
indy_parse_get_cred_def_response,
indy_build_node_request,
indy_build_get_validator_info_request,
indy_build_get_txn_request,
indy_build_pool_config_request,
indy_build_pool_restart_request,
indy_build_pool_upgrade_request,
indy_build_revoc_reg_def_request,
indy_build_get_revoc_reg_def_request,
indy_parse_get_revoc_reg_def_response,
indy_build_revoc_reg_entry_request,
indy_build_get_revoc_reg_request,
indy_parse_get_revoc_reg_response,
indy_build_get_revoc_reg_delta_request,
indy_parse_get_revoc_reg_delta_response};

pub fn sign_and_submit_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let pool_handle: i32 = try!(args[0].decode());
  let wallet_handle: i32 = try!(args[1].decode());    
  let submitter_did: String = try!(args[2].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let request_json: String = try!(args[3].decode()); 
    let c_request_json = CString::new(request_json).unwrap();

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_sign_and_submit_request(command_handle, pool_handle,wallet_handle,c_submitter_did.as_ptr(),c_request_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn submit_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let pool_handle: i32 = try!(args[0].decode());    let request_json: String = try!(args[1].decode()); 
    let c_request_json = CString::new(request_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_submit_request(command_handle, pool_handle,c_request_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn submit_action<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let pool_handle: i32 = try!(args[0].decode());    let request_json: String = try!(args[1].decode()); 
    let c_request_json = CString::new(request_json).unwrap();
    let nodes: String = try!(args[2].decode()); 
    let c_nodes = CString::new(nodes).unwrap();
let timeout: i32 = try!(args[3].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_submit_action(command_handle, pool_handle,c_request_json.as_ptr(),c_nodes.as_ptr(),timeout, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn sign_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let request_json: String = try!(args[2].decode()); 
    let c_request_json = CString::new(request_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_sign_request(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_request_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn multi_sign_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let request_json: String = try!(args[2].decode()); 
    let c_request_json = CString::new(request_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_multi_sign_request(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_request_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_ddo_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let target_did: String = try!(args[1].decode()); 
    let c_target_did = CString::new(target_did).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_ddo_request(command_handle, c_submitter_did.as_ptr(),c_target_did.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_nym_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let target_did: String = try!(args[1].decode()); 
    let c_target_did = CString::new(target_did).unwrap();
    let verkey: String = try!(args[2].decode()); 
    let c_verkey = CString::new(verkey).unwrap();
    let x_alias: String = try!(args[3].decode()); 
    let c_alias = CString::new(x_alias).unwrap();
    let role: String = try!(args[4].decode()); 
    let c_role = CString::new(role).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_nym_request(command_handle, c_submitter_did.as_ptr(),c_target_did.as_ptr(),c_verkey.as_ptr(),c_alias.as_ptr(),c_role.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_nym_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let target_did: String = try!(args[1].decode()); 
    let c_target_did = CString::new(target_did).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_nym_request(command_handle, c_submitter_did.as_ptr(),c_target_did.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_attrib_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let target_did: String = try!(args[1].decode()); 
    let c_target_did = CString::new(target_did).unwrap();
    let hash: String = try!(args[2].decode()); 
    let c_hash = CString::new(hash).unwrap();
    let raw: String = try!(args[3].decode()); 
    let c_raw = CString::new(raw).unwrap();
    let enc: String = try!(args[4].decode()); 
    let c_enc = CString::new(enc).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_attrib_request(command_handle, c_submitter_did.as_ptr(),c_target_did.as_ptr(),c_hash.as_ptr(),c_raw.as_ptr(),c_enc.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_attrib_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let target_did: String = try!(args[1].decode()); 
    let c_target_did = CString::new(target_did).unwrap();
    let raw: String = try!(args[2].decode()); 
    let c_raw = CString::new(raw).unwrap();
    let hash: String = try!(args[3].decode()); 
    let c_hash = CString::new(hash).unwrap();
    let enc: String = try!(args[4].decode()); 
    let c_enc = CString::new(enc).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_attrib_request(command_handle, c_submitter_did.as_ptr(),c_target_did.as_ptr(),c_raw.as_ptr(),c_hash.as_ptr(),c_enc.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_schema_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let data: String = try!(args[1].decode()); 
    let c_data = CString::new(data).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_schema_request(command_handle, c_submitter_did.as_ptr(),c_data.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_schema_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let id: String = try!(args[1].decode()); 
    let c_id = CString::new(id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_schema_request(command_handle, c_submitter_did.as_ptr(),c_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_schema_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let get_schema_response: String = try!(args[0].decode()); 
    let c_get_schema_response = CString::new(get_schema_response).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_parse_get_schema_response(command_handle, c_get_schema_response.as_ptr(), cb);

  let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_cred_def_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let data: String = try!(args[1].decode()); 
    let c_data = CString::new(data).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_cred_def_request(command_handle, c_submitter_did.as_ptr(),c_data.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_cred_def_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let id: String = try!(args[1].decode()); 
    let c_id = CString::new(id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_cred_def_request(command_handle, c_submitter_did.as_ptr(),c_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_cred_def_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let get_cred_def_response: String = try!(args[0].decode()); 
    let c_get_cred_def_response = CString::new(get_cred_def_response).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_parse_get_cred_def_response(command_handle, c_get_cred_def_response.as_ptr(), cb);

  let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_node_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let target_did: String = try!(args[1].decode()); 
    let c_target_did = CString::new(target_did).unwrap();
    let data: String = try!(args[2].decode()); 
    let c_data = CString::new(data).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_node_request(command_handle, c_submitter_did.as_ptr(),c_target_did.as_ptr(),c_data.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_validator_info_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_validator_info_request(command_handle, c_submitter_did.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_txn_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let ledger_type: String = try!(args[1].decode()); 
    let c_ledger_type = CString::new(ledger_type).unwrap();
    let seq_no: i32 = try!(args[2].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_txn_request(command_handle, c_submitter_did.as_ptr(),c_ledger_type.as_ptr(),seq_no, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_pool_config_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let writes: bool = try!(args[1].decode());
    let force: bool = try!(args[2].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_pool_config_request(command_handle, c_submitter_did.as_ptr(),writes,force, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_pool_restart_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let action: String = try!(args[1].decode()); 
    let c_action = CString::new(action).unwrap();
    let datetime: String = try!(args[2].decode()); 
    let c_datetime = CString::new(datetime).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_pool_restart_request(command_handle, c_submitter_did.as_ptr(),c_action.as_ptr(),c_datetime.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_pool_upgrade_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let name: String = try!(args[1].decode()); 
    let c_name = CString::new(name).unwrap();
    let version: String = try!(args[2].decode()); 
    let c_version = CString::new(version).unwrap();
    let action: String = try!(args[3].decode()); 
    let c_action = CString::new(action).unwrap();
    let sha256: String = try!(args[4].decode()); 
    let c_sha256 = CString::new(sha256).unwrap();
    let timeout: i32 = try!(args[5].decode());    
    let schedule: String = try!(args[6].decode()); 
    let c_schedule = CString::new(schedule).unwrap();
    let justification: String = try!(args[7].decode()); 
    let c_justification = CString::new(justification).unwrap();
    let reinstall: bool = try!(args[8].decode());
    let force: bool = try!(args[9].decode());
    let package: String = try!(args[10].decode()); 
    let c_package = CString::new(package).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_pool_upgrade_request(command_handle, c_submitter_did.as_ptr(),c_name.as_ptr(),c_version.as_ptr(),c_action.as_ptr(),c_sha256.as_ptr(),timeout,c_schedule.as_ptr(),c_justification.as_ptr(),reinstall,force,c_package.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_revoc_reg_def_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let data: String = try!(args[1].decode()); 
    let c_data = CString::new(data).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_revoc_reg_def_request(command_handle, c_submitter_did.as_ptr(),c_data.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_revoc_reg_def_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let id: String = try!(args[1].decode()); 
    let c_id = CString::new(id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_revoc_reg_def_request(command_handle, c_submitter_did.as_ptr(),c_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_revoc_reg_def_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let get_revoc_reg_def_response: String = try!(args[0].decode()); 
    let c_get_revoc_reg_def_response = CString::new(get_revoc_reg_def_response).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_parse_get_revoc_reg_def_response(command_handle, c_get_revoc_reg_def_response.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_revoc_reg_entry_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let revoc_reg_def_id: String = try!(args[1].decode()); 
    let c_revoc_reg_def_id = CString::new(revoc_reg_def_id).unwrap();
    let rev_def_type: String = try!(args[2].decode()); 
    let c_rev_def_type = CString::new(rev_def_type).unwrap();
    let value: String = try!(args[3].decode()); 
    let c_value = CString::new(value).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_revoc_reg_entry_request(command_handle, c_submitter_did.as_ptr(),c_revoc_reg_def_id.as_ptr(),c_rev_def_type.as_ptr(),c_value.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_revoc_reg_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let revoc_reg_def_id: String = try!(args[1].decode()); 
    let c_revoc_reg_def_id = CString::new(revoc_reg_def_id).unwrap();
    let timestamp: i64 = try!(args[2].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_revoc_reg_request(command_handle, c_submitter_did.as_ptr(),c_revoc_reg_def_id.as_ptr(),timestamp, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_revoc_reg_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let get_revoc_reg_response: String = try!(args[0].decode()); 
    let c_get_revoc_reg_response = CString::new(get_revoc_reg_response).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string_timestamp();

    let err = indy_parse_get_revoc_reg_response(command_handle, c_get_revoc_reg_response.as_ptr(), cb);

    let response = result_to_string_string_timestamp(err, receiver);

    Ok(response.encode(env))
}

pub fn build_get_revoc_reg_delta_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let submitter_did: String = try!(args[0].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let revoc_reg_def_id: String = try!(args[1].decode()); 
    let c_revoc_reg_def_id = CString::new(revoc_reg_def_id).unwrap();
    let from: i64 = try!(args[2].decode());
    let x_to: i64 = try!(args[3].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_revoc_reg_delta_request(command_handle,c_submitter_did.as_ptr(),c_revoc_reg_def_id.as_ptr(),from,x_to, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_revoc_reg_delta_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let get_revoc_reg_delta_response: String = try!(args[0].decode()); 
    let c_get_revoc_reg_delta_response = CString::new(get_revoc_reg_delta_response).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string_timestamp();

    let err = indy_parse_get_revoc_reg_delta_response(command_handle, c_get_revoc_reg_delta_response.as_ptr(), cb);

    let response = result_to_string_string_timestamp(err, receiver);

    Ok(response.encode(env))
}



