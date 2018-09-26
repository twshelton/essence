use std::ffi::CString;
use rustler::{Env, Term, NifResult, Encoder};
use utils::ex_results::{result_to_string, result_to_int, result_to_empty};
use utils::callbacks;
use indy::api::non_secrets::{indy_add_wallet_record,
indy_update_wallet_record_value,
indy_update_wallet_record_tags,
indy_add_wallet_record_tags,
indy_delete_wallet_record_tags,
indy_delete_wallet_record,
indy_get_wallet_record,
indy_open_wallet_search,
indy_fetch_wallet_search_next_records,
indy_close_wallet_search};

pub fn add_wallet_record<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();
    let value: String = try!(args[3].decode()); 
    let c_value = CString::new(value).unwrap();
    let tags_json: String = try!(args[4].decode()); 
    let c_tags_json = CString::new(tags_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_add_wallet_record(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(),c_value.as_ptr(),c_tags_json.as_ptr(), cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn update_wallet_record_value<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();
    let value: String = try!(args[3].decode()); 
    let c_value = CString::new(value).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_update_wallet_record_value(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(),c_value.as_ptr(), cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn update_wallet_record_tags<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();
    let tags_json: String = try!(args[3].decode()); 
    let c_tags_json = CString::new(tags_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_update_wallet_record_tags(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(),c_tags_json.as_ptr(), cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn add_wallet_record_tags<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();
    let tags_json: String = try!(args[3].decode()); 
    let c_tags_json = CString::new(tags_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_add_wallet_record_tags(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(),c_tags_json.as_ptr(), cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn delete_wallet_record_tags<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();
    let tag_names_json: String = try!(args[3].decode()); 
    let c_tag_names_json = CString::new(tag_names_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_delete_wallet_record_tags(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(),c_tag_names_json.as_ptr(), cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn delete_wallet_record<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_delete_wallet_record(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(), cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn get_wallet_record<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let id: String = try!(args[2].decode()); 
    let c_id = CString::new(id).unwrap();
    let options_json: String = try!(args[3].decode()); 
    let c_options_json = CString::new(options_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_get_wallet_record(command_handle, wallet_handle,c_type_.as_ptr(),c_id.as_ptr(),c_options_json.as_ptr(), cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn open_wallet_search<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());    let type_: String = try!(args[1].decode()); 
    let c_type_ = CString::new(type_).unwrap();
    let query_json: String = try!(args[2].decode()); 
    let c_query_json = CString::new(query_json).unwrap();
    let options_json: String = try!(args[3].decode()); 
    let c_options_json = CString::new(options_json).unwrap();


  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_i32();

  let err = indy_open_wallet_search(command_handle, wallet_handle,c_type_.as_ptr(),c_query_json.as_ptr(),c_options_json.as_ptr(), cb);

  let response = result_to_int(err, receiver); 

  Ok(response.encode(env))
}

pub fn fetch_wallet_search_next_records<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_handle: i32 = try!(args[0].decode());
  let wallet_search_handle: i32 = try!(args[1].decode());
  let count: usize = try!(args[2].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

  let err = indy_fetch_wallet_search_next_records(command_handle, wallet_handle,wallet_search_handle,count, cb);

  let response = result_to_string(err, receiver);

  Ok(response.encode(env))
}

pub fn close_wallet_search<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let wallet_search_handle: i32 = try!(args[0].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_close_wallet_search(command_handle, wallet_search_handle, cb);

let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}
