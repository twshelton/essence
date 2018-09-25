use std::ffi::CString;
use rustler::{Env, Term, NifResult, Encoder};
use utils::ex_results::{result_to_string, result_to_empty, result_to_bool};
use utils::callbacks;
use indy::api::pairwise::{indy_is_pairwise_exists,
indy_create_pairwise,
indy_list_pairwise,
indy_get_pairwise,
indy_set_pairwise_metadata};

pub fn is_pairwise_exists<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let their_did: String = try!(args[1].decode()); 
    let c_their_did = CString::new(their_did).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bool();

  let err = indy_is_pairwise_exists(command_handle, wallet_handle,c_their_did.as_ptr(), cb);

  let response = result_to_bool(err, receiver); 

  Ok(response.encode(env))
}

pub fn create_pairwise<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let their_did: String = try!(args[1].decode()); 
    let c_their_did = CString::new(their_did).unwrap();
    let my_did: String = try!(args[2].decode()); 
    let c_my_did = CString::new(my_did).unwrap();
    let metadata: String = try!(args[3].decode()); 
    let c_metadata = CString::new(metadata).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_create_pairwise(command_handle, wallet_handle,c_their_did.as_ptr(),c_my_did.as_ptr(),c_metadata.as_ptr(), cb);

  /* [] */
let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn list_pairwise<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_list_pairwise(command_handle, wallet_handle, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn get_pairwise<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let their_did: String = try!(args[1].decode()); 
    let c_their_did = CString::new(their_did).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_get_pairwise(command_handle, wallet_handle,c_their_did.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn set_pairwise_metadata<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let their_did: String = try!(args[1].decode()); 
    let c_their_did = CString::new(their_did).unwrap();
    let metadata: String = try!(args[2].decode()); 
    let c_metadata = CString::new(metadata).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_set_pairwise_metadata(command_handle, wallet_handle,c_their_did.as_ptr(),c_metadata.as_ptr(), cb);

  /* [] */
let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}



