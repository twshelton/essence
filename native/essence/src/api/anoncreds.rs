   use std::ffi::CString;
   use rustler::{Env, Term, NifResult, Encoder};
   use utils::ex_results::{result_to_string, result_to_int, result_to_empty, result_to_bool, result_to_handle_count,
   result_to_string_string, result_to_string_string_string};
   use utils::callbacks;
   use indy::api::anoncreds::{indy_issuer_create_schema,
       indy_issuer_create_and_store_credential_def,
       indy_issuer_create_and_store_revoc_reg,
       indy_issuer_create_credential_offer,
       indy_issuer_create_credential,
       indy_issuer_revoke_credential,
       indy_issuer_merge_revocation_registry_deltas,
       indy_prover_create_master_secret,
       indy_prover_create_credential_req,
       indy_prover_store_credential,
       indy_prover_get_credential,
       indy_prover_search_credentials,
       indy_prover_fetch_credentials,
       indy_prover_close_credentials_search,
       indy_prover_search_credentials_for_proof_req,
       indy_prover_fetch_credentials_for_proof_req,
       indy_prover_close_credentials_search_for_proof_req,
       indy_prover_create_proof,
       indy_verifier_verify_proof,
       indy_create_revocation_state,
       indy_update_revocation_state};

     pub fn issuer_create_schema<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let issuer_did: String = try!(args[0].decode()); 
    let c_issuer_did = CString::new(issuer_did).unwrap();
    let name: String = try!(args[1].decode()); 
    let c_name = CString::new(name).unwrap();
    let version: String = try!(args[2].decode()); 
    let c_version = CString::new(version).unwrap();
    let attrs: String = try!(args[3].decode()); 
    let c_attrs = CString::new(attrs).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_issuer_create_schema(command_handle, c_issuer_did.as_ptr(),c_name.as_ptr(),c_version.as_ptr(),c_attrs.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn issuer_create_and_store_credential_def<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let issuer_did: String = try!(args[1].decode()); 
    let c_issuer_did = CString::new(issuer_did).unwrap();
    let schema_json: String = try!(args[2].decode()); 
    let c_schema_json = CString::new(schema_json).unwrap();
    let tag: String = try!(args[3].decode()); 
    let c_tag = CString::new(tag).unwrap();
    let signature_type: String = try!(args[4].decode()); 
    let c_signature_type = CString::new(signature_type).unwrap();
    let config_json: String = try!(args[5].decode()); 
    let c_config_json = CString::new(config_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_issuer_create_and_store_credential_def(command_handle, wallet_handle,c_issuer_did.as_ptr(),c_schema_json.as_ptr(),c_tag.as_ptr(),c_signature_type.as_ptr(),c_config_json.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn issuer_create_and_store_revoc_reg<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let issuer_did: String = try!(args[1].decode()); 
    let c_issuer_did = CString::new(issuer_did).unwrap();
    let revoc_def_type: String = try!(args[2].decode()); 
    let c_revoc_def_type = CString::new(revoc_def_type).unwrap();
    let tag: String = try!(args[3].decode()); 
    let c_tag = CString::new(tag).unwrap();
    let cred_def_id: String = try!(args[4].decode()); 
    let c_cred_def_id = CString::new(cred_def_id).unwrap();
    let config_json: String = try!(args[5].decode()); 
    let c_config_json = CString::new(config_json).unwrap();
let tails_writer_handle: i32 = try!(args[6].decode());

let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string_string();


  let err = indy_issuer_create_and_store_revoc_reg(command_handle, wallet_handle,c_issuer_did.as_ptr(),c_revoc_def_type.as_ptr(),c_tag.as_ptr(),c_cred_def_id.as_ptr(),c_config_json.as_ptr(),tails_writer_handle, cb);

let response = result_to_string_string_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn issuer_create_credential_offer<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let cred_def_id: String = try!(args[1].decode()); 
    let c_cred_def_id = CString::new(cred_def_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_issuer_create_credential_offer(command_handle, wallet_handle,c_cred_def_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn issuer_create_credential<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let cred_offer_json: String = try!(args[1].decode()); 
    let c_cred_offer_json = CString::new(cred_offer_json).unwrap();
    let cred_req_json: String = try!(args[2].decode()); 
    let c_cred_req_json = CString::new(cred_req_json).unwrap();
    let cred_values_json: String = try!(args[3].decode()); 
    let c_cred_values_json = CString::new(cred_values_json).unwrap();
    let rev_reg_id: String = try!(args[4].decode()); 
    let c_rev_reg_id = CString::new(rev_reg_id).unwrap();
let blob_storage_reader_handle: i32 = try!(args[5].decode());

  /* [cred_json: :char, cred_revoc_id: :char, revoc_reg_delta_json: :char] */
let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string_string();


  let err = indy_issuer_create_credential(command_handle, wallet_handle,c_cred_offer_json.as_ptr(),c_cred_req_json.as_ptr(),c_cred_values_json.as_ptr(),c_rev_reg_id.as_ptr(),blob_storage_reader_handle, cb);

  /* [cred_json: :char, cred_revoc_id: :char, revoc_reg_delta_json: :char] */
let response = result_to_string_string_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn issuer_revoke_credential<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());let blob_storage_reader_cfg_handle: i32 = try!(args[1].decode());    let rev_reg_id: String = try!(args[2].decode()); 
    let c_rev_reg_id = CString::new(rev_reg_id).unwrap();
    let cred_revoc_id: String = try!(args[3].decode()); 
    let c_cred_revoc_id = CString::new(cred_revoc_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_issuer_revoke_credential(command_handle, wallet_handle,blob_storage_reader_cfg_handle,c_rev_reg_id.as_ptr(),c_cred_revoc_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn issuer_merge_revocation_registry_deltas<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let rev_reg_delta_json: String = try!(args[0].decode()); 
    let c_rev_reg_delta_json = CString::new(rev_reg_delta_json).unwrap();
    let other_rev_reg_delta_json: String = try!(args[1].decode()); 
    let c_other_rev_reg_delta_json = CString::new(other_rev_reg_delta_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_issuer_merge_revocation_registry_deltas(command_handle, c_rev_reg_delta_json.as_ptr(),c_other_rev_reg_delta_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_create_master_secret<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let master_secret_id: String = try!(args[1].decode()); 
    let c_master_secret_id = CString::new(master_secret_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_prover_create_master_secret(command_handle, wallet_handle,c_master_secret_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_create_credential_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let prover_did: String = try!(args[1].decode()); 
    let c_prover_did = CString::new(prover_did).unwrap();
    let cred_offer_json: String = try!(args[2].decode()); 
    let c_cred_offer_json = CString::new(cred_offer_json).unwrap();
    let cred_def_json: String = try!(args[3].decode()); 
    let c_cred_def_json = CString::new(cred_def_json).unwrap();
    let master_secret_id: String = try!(args[4].decode()); 
    let c_master_secret_id = CString::new(master_secret_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_prover_create_credential_req(command_handle, wallet_handle,c_prover_did.as_ptr(),c_cred_offer_json.as_ptr(),c_cred_def_json.as_ptr(),c_master_secret_id.as_ptr(), cb);

  /* [cred_req_json: :char, cred_req_metadata_json: :char] */
let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_store_credential<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let cred_id: String = try!(args[1].decode()); 
    let c_cred_id = CString::new(cred_id).unwrap();
    let cred_req_metadata_json: String = try!(args[2].decode()); 
    let c_cred_req_metadata_json = CString::new(cred_req_metadata_json).unwrap();
    let cred_json: String = try!(args[3].decode()); 
    let c_cred_json = CString::new(cred_json).unwrap();
    let cred_def_json: String = try!(args[4].decode()); 
    let c_cred_def_json = CString::new(cred_def_json).unwrap();
    let rev_reg_def_json: String = try!(args[5].decode()); 
    let c_rev_reg_def_json = CString::new(rev_reg_def_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_prover_store_credential(command_handle, wallet_handle,c_cred_id.as_ptr(),c_cred_req_metadata_json.as_ptr(),c_cred_json.as_ptr(),c_cred_def_json.as_ptr(),c_rev_reg_def_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_get_credential<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let cred_id: String = try!(args[1].decode()); 
    let c_cred_id = CString::new(cred_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_prover_get_credential(command_handle, wallet_handle,c_cred_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_search_credentials<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let query_json: String = try!(args[1].decode()); 
    let c_query_json = CString::new(query_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_handle_count();

  let err = indy_prover_search_credentials(command_handle, wallet_handle,c_query_json.as_ptr(), cb);

    let response = result_to_handle_count(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_fetch_credentials<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let search_handle: i32 = try!(args[0].decode());
  let count: usize = try!(args[1].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_prover_fetch_credentials(command_handle, search_handle,count, cb);

let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_close_credentials_search<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let search_handle: i32 = try!(args[0].decode());

let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();


  let err = indy_prover_close_credentials_search(command_handle, search_handle, cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_search_credentials_for_proof_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let proof_request_json: String = try!(args[1].decode()); 
    let c_proof_request_json = CString::new(proof_request_json).unwrap();
    let extra_query_json: String = try!(args[2].decode()); 
    let c_extra_query_json = CString::new(extra_query_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_i32();

  let err = indy_prover_search_credentials_for_proof_req(command_handle, wallet_handle,c_proof_request_json.as_ptr(),c_extra_query_json.as_ptr(), cb);

  let response = result_to_int(err, receiver); 

  Ok(response.encode(env))
  }

  pub fn prover_fetch_credentials_for_proof_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let search_handle: i32 = try!(args[0].decode());    let item_referent: String = try!(args[1].decode()); 
    let c_item_referent = CString::new(item_referent).unwrap();
    let count: usize = try!(args[1].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_prover_fetch_credentials_for_proof_req(command_handle, search_handle,c_item_referent.as_ptr(),count, cb);

let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_close_credentials_search_for_proof_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let search_handle: i32 = try!(args[0].decode());

let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();


  let err = indy_prover_close_credentials_search_for_proof_req(command_handle, search_handle, cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
  }

  pub fn prover_create_proof<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let proof_req_json: String = try!(args[1].decode()); 
    let c_proof_req_json = CString::new(proof_req_json).unwrap();
    let requested_credentials_json: String = try!(args[2].decode()); 
    let c_requested_credentials_json = CString::new(requested_credentials_json).unwrap();
    let master_secret_id: String = try!(args[3].decode()); 
    let c_master_secret_id = CString::new(master_secret_id).unwrap();
    let schemas_json: String = try!(args[4].decode()); 
    let c_schemas_json = CString::new(schemas_json).unwrap();
    let credential_defs_json: String = try!(args[5].decode()); 
    let c_credential_defs_json = CString::new(credential_defs_json).unwrap();
    let rev_states_json: String = try!(args[6].decode()); 
    let c_rev_states_json = CString::new(rev_states_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_prover_create_proof(command_handle, wallet_handle,c_proof_req_json.as_ptr(),c_requested_credentials_json.as_ptr(),c_master_secret_id.as_ptr(),c_schemas_json.as_ptr(),c_credential_defs_json.as_ptr(),c_rev_states_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn verifier_verify_proof<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let proof_request_json: String = try!(args[0].decode()); 
    let c_proof_request_json = CString::new(proof_request_json).unwrap();
    let proof_json: String = try!(args[1].decode()); 
    let c_proof_json = CString::new(proof_json).unwrap();
    let schemas_json: String = try!(args[2].decode()); 
    let c_schemas_json = CString::new(schemas_json).unwrap();
    let credential_defs_json: String = try!(args[3].decode()); 
    let c_credential_defs_json = CString::new(credential_defs_json).unwrap();
    let rev_reg_defs_json: String = try!(args[4].decode()); 
    let c_rev_reg_defs_json = CString::new(rev_reg_defs_json).unwrap();
    let rev_regs_json: String = try!(args[5].decode()); 
    let c_rev_regs_json = CString::new(rev_regs_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bool();

  let err = indy_verifier_verify_proof(command_handle, c_proof_request_json.as_ptr(),c_proof_json.as_ptr(),c_schemas_json.as_ptr(),c_credential_defs_json.as_ptr(),c_rev_reg_defs_json.as_ptr(),c_rev_regs_json.as_ptr(), cb);

  let response = result_to_bool(err, receiver); 

  Ok(response.encode(env))
  }

  pub fn create_revocation_state<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let blob_storage_reader_handle: i32 = try!(args[0].decode());    let rev_reg_def_json: String = try!(args[1].decode()); 
    let c_rev_reg_def_json = CString::new(rev_reg_def_json).unwrap();
    let rev_reg_delta_json: String = try!(args[2].decode()); 
    let c_rev_reg_delta_json = CString::new(rev_reg_delta_json).unwrap();
let timestamp: u64 = try!(args[3].decode());    let cred_rev_id: String = try!(args[4].decode()); 
    let c_cred_rev_id = CString::new(cred_rev_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_create_revocation_state(command_handle, blob_storage_reader_handle,c_rev_reg_def_json.as_ptr(),c_rev_reg_delta_json.as_ptr(),timestamp,c_cred_rev_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }

  pub fn update_revocation_state<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let blob_storage_reader_handle: i32 = try!(args[0].decode());    let rev_state_json: String = try!(args[1].decode()); 
    let c_rev_state_json = CString::new(rev_state_json).unwrap();
    let rev_reg_def_json: String = try!(args[2].decode()); 
    let c_rev_reg_def_json = CString::new(rev_reg_def_json).unwrap();
    let rev_reg_delta_json: String = try!(args[3].decode()); 
    let c_rev_reg_delta_json = CString::new(rev_reg_delta_json).unwrap();
let timestamp: u64 = try!(args[4].decode());    let cred_rev_id: String = try!(args[5].decode()); 
    let c_cred_rev_id = CString::new(cred_rev_id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_update_revocation_state(command_handle, blob_storage_reader_handle,c_rev_state_json.as_ptr(),c_rev_reg_def_json.as_ptr(),c_rev_reg_delta_json.as_ptr(),timestamp,c_cred_rev_id.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
  }



