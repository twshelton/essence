use std::ffi::CString;
use rustler::{Env, Term, NifResult, Encoder};
use utils::ex_results::{result_to_string, result_to_string_string};
use utils::callbacks;
use indy::api::payments::{//indy_register_payment_method,
indy_create_payment_address,
indy_list_payment_addresses,
indy_add_request_fees,
indy_parse_response_with_fees,
indy_build_get_payment_sources_request,
indy_parse_get_payment_sources_response,
indy_build_payment_req,
indy_parse_payment_response,
indy_build_mint_req,
indy_build_set_txn_fees_req,
indy_build_get_txn_fees_req,
indy_parse_get_txn_fees_response,
indy_build_verify_payment_req,
indy_parse_verify_payment_response};

pub fn register_payment_method<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
/*
   TODO: fix this one ... requires more work for the Option arguments
   let payment_method: String = try!(args[0].decode()); 
   let c_payment_method = CString::new(payment_method).unwrap();
   Dont know what to do with this: OptionCreatePaymentAddressCB
   Dont know what to do with this: OptionAddRequestFeesCB
   Dont know what to do with this: OptionParseResponseWithFeesCB
   Dont know what to do with this: OptionBuildGetPaymentSourcesRequestCB
   Dont know what to do with this: OptionParseGetPaymentSourcesResponseCB
   Dont know what to do with this: OptionBuildPaymentReqCB
   Dont know what to do with this: OptionParsePaymentResponseCB
   Dont know what to do with this: OptionBuildMintReqCB
   Dont know what to do with this: OptionBuildSetTxnFeesReqCB
   Dont know what to do with this: OptionBuildGetTxnFeesReqCB
   Dont know what to do with this: OptionParseGetTxnFeesResponseCB
   Dont know what to do with this: OptionBuildVerifyPaymentReqCB
   Dont know what to do with this: OptionParseVerifyPaymentResponseCB

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_register_payment_method(command_handle, c_payment_method.as_ptr(),create_payment_address,add_request_fees,parse_response_with_fees,build_get_payment_sources_request,parse_get_payment_sources_response,build_payment_req,parse_payment_response,build_mint_req,build_set_txn_fees_req,build_get_txn_fees_req,parse_get_txn_fees_response,build_verify_payment_req,parse_verify_payment_response, cb);

    let response = result_to_empty(err, receiver);
*/
  Ok(0.encode(env))
}

pub fn create_payment_address<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let payment_method: String = try!(args[1].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let config: String = try!(args[2].decode()); 
    let c_config = CString::new(config).unwrap();

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_create_payment_address(command_handle, wallet_handle,c_payment_method.as_ptr(),c_config.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn list_payment_addresses<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_list_payment_addresses(command_handle, wallet_handle, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn add_request_fees<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let req_json: String = try!(args[2].decode()); 
    let c_req_json = CString::new(req_json).unwrap();
    let inputs_json: String = try!(args[3].decode()); 
    let c_inputs_json = CString::new(inputs_json).unwrap();
    let outputs_json: String = try!(args[4].decode()); 
    let c_outputs_json = CString::new(outputs_json).unwrap();
    let extra: String = try!(args[5].decode()); 
    let c_extra = CString::new(extra).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_add_request_fees(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_req_json.as_ptr(),c_inputs_json.as_ptr(),c_outputs_json.as_ptr(),c_extra.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_response_with_fees<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let payment_method: String = try!(args[0].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let resp_json: String = try!(args[1].decode()); 
    let c_resp_json = CString::new(resp_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_parse_response_with_fees(command_handle, c_payment_method.as_ptr(),c_resp_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_payment_sources_request<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let payment_address: String = try!(args[2].decode()); 
    let c_payment_address = CString::new(payment_address).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_build_get_payment_sources_request(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_payment_address.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_payment_sources_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let payment_method: String = try!(args[0].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let resp_json: String = try!(args[1].decode()); 
    let c_resp_json = CString::new(resp_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_parse_get_payment_sources_response(command_handle, c_payment_method.as_ptr(),c_resp_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_payment_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let inputs_json: String = try!(args[2].decode()); 
    let c_inputs_json = CString::new(inputs_json).unwrap();
    let outputs_json: String = try!(args[3].decode()); 
    let c_outputs_json = CString::new(outputs_json).unwrap();
    let extra: String = try!(args[4].decode()); 
    let c_extra = CString::new(extra).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_build_payment_req(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_inputs_json.as_ptr(),c_outputs_json.as_ptr(),c_extra.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_payment_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let payment_method: String = try!(args[0].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let resp_json: String = try!(args[1].decode()); 
    let c_resp_json = CString::new(resp_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_parse_payment_response(command_handle, c_payment_method.as_ptr(),c_resp_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_mint_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let outputs_json: String = try!(args[2].decode()); 
    let c_outputs_json = CString::new(outputs_json).unwrap();
    let extra: String = try!(args[3].decode()); 
    let c_extra = CString::new(extra).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_build_mint_req(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_outputs_json.as_ptr(),c_extra.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_set_txn_fees_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let payment_method: String = try!(args[2].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let fees_json: String = try!(args[3].decode()); 
    let c_fees_json = CString::new(fees_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_set_txn_fees_req(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_payment_method.as_ptr(),c_fees_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_get_txn_fees_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let payment_method: String = try!(args[2].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_build_get_txn_fees_req(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_payment_method.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_get_txn_fees_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let payment_method: String = try!(args[0].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let resp_json: String = try!(args[1].decode()); 
    let c_resp_json = CString::new(resp_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_parse_get_txn_fees_response(command_handle, c_payment_method.as_ptr(),c_resp_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn build_verify_payment_req<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let submitter_did: String = try!(args[1].decode()); 
    let c_submitter_did = CString::new(submitter_did).unwrap();
    let receipt: String = try!(args[2].decode()); 
    let c_receipt = CString::new(receipt).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

  let err = indy_build_verify_payment_req(command_handle, wallet_handle,c_submitter_did.as_ptr(),c_receipt.as_ptr(), cb);

let response = result_to_string_string(err, receiver);

  Ok(response.encode(env))
}

pub fn parse_verify_payment_response<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

      let payment_method: String = try!(args[0].decode()); 
    let c_payment_method = CString::new(payment_method).unwrap();
    let resp_json: String = try!(args[1].decode()); 
    let c_resp_json = CString::new(resp_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_parse_verify_payment_response(command_handle, c_payment_method.as_ptr(),c_resp_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}



