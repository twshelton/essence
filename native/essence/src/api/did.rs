
use std::ffi::CString;
use rustler::{Env, Term, NifResult, Encoder};
use utils::results::{result_to_string, result_to_empty, result_to_string_string};
use utils::atoms;
use utils::callbacks;
use
indy::api::did::{
    indy_create_and_store_my_did,
    indy_replace_keys_start,
    indy_replace_keys_apply,
    indy_set_did_metadata,
    indy_get_did_metadata,
    indy_get_my_did_with_meta,
    indy_list_my_dids_with_meta,
    indy_abbreviate_verkey,
    indy_set_endpoint_for_did,
    indy_get_endpoint_for_did,
    indy_key_for_did,
    indy_key_for_local_did,
    indy_store_their_did
    };

pub fn create_and_store_my_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did_json: String = try!(args[1].decode());
    let c_did_json = CString::new(did_json).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

    let err = indy_create_and_store_my_did(command_handle, wallet_handle, c_did_json.as_ptr(), cb);

    let response = match result_to_string_string(err, receiver) {
      Ok((did, verkey)) => (atoms::ok(), format!("Success! {:?} {:?}", did, verkey)),
      Err(err) => (atoms::error(), format!("{:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn replace_keys_start<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let identity_json: String = try!(args[2].decode());
    let c_did = CString::new(did).unwrap();
    let c_identity_json = CString::new(identity_json).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_replace_keys_start(command_handle, wallet_handle, c_did.as_ptr(), c_identity_json.as_ptr(), cb);

    let response = match result_to_string(err, receiver) {
        Ok(verkey) => (atoms::ok(), format!("Success! {:?}", verkey)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn replace_keys_apply<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let c_did = CString::new(did).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_replace_keys_apply(command_handle, wallet_handle, c_did.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn set_did_metadata<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let metadata: String = try!(args[2].decode());
    let c_did = CString::new(did).unwrap();
    let c_metadata = CString::new(metadata).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_set_did_metadata(command_handle, wallet_handle, c_did.as_ptr(), c_metadata.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn get_did_metadata<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let c_did = CString::new(did).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_get_did_metadata(command_handle, wallet_handle, c_did.as_ptr(), cb);

    let response = match result_to_string(err, receiver) {
        Ok(metadata) => (atoms::ok(), format!("Success! {:?}", metadata)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn get_my_did_with_meta<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let c_did = CString::new(did).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_get_my_did_with_meta(command_handle, wallet_handle, c_did.as_ptr(), cb);

    let response = match result_to_string(err, receiver) {
        Ok(metadata) => (atoms::ok(), format!("Success! {:?}", metadata)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn list_my_dids_with_meta<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_list_my_dids_with_meta(command_handle, wallet_handle, cb);

    let response = match result_to_string(err, receiver) {
        Ok(metadata) => (atoms::ok(), format!("Success! {:?}", metadata)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn abbreviate_verkey<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let did: String = try!(args[0].decode());
    let c_did = CString::new(did).unwrap();
    let verkey: String = try!(args[1].decode());
    let c_verkey = CString::new(verkey).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_abbreviate_verkey(command_handle, c_did.as_ptr(), c_verkey.as_ptr(), cb);

    let response = match result_to_string(err, receiver) {
        Ok(verkey) => (atoms::ok(), format!("Success! {:?}", verkey)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn get_endpoint_for_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let pool_handle: i32 = try!(args[1].decode());
    let did: String = try!(args[2].decode());
    let c_did = CString::new(did).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string_string();

    let err = indy_get_endpoint_for_did(command_handle, wallet_handle, pool_handle, c_did.as_ptr(), cb);

    let response = match result_to_string_string(err, receiver) {
        Ok((address, transport_vk)) => (atoms::ok(), format!("Success! {:?} {:?}", address, transport_vk)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn set_endpoint_for_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let c_did = CString::new(did).unwrap();
    let address: String = try!(args[2].decode());
    let c_address = CString::new(address).unwrap();
    let transport_key: String = try!(args[3].decode());
    let c_transport_key = CString::new(transport_key).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_set_endpoint_for_did(command_handle, wallet_handle, c_did.as_ptr(), c_address.as_ptr(),
    c_transport_key.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn key_for_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let pool_handle: i32 = try!(args[0].decode());
    let wallet_handle: i32 = try!(args[1].decode());
    let did: String = try!(args[2].decode());
    let c_did = CString::new(did).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_key_for_did(command_handle, pool_handle, wallet_handle, c_did.as_ptr(), cb);

    let response = match result_to_string(err, receiver) {
        Ok(key) => (atoms::ok(), format!("Success! {:?}", key)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn key_for_local_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let did: String = try!(args[1].decode());
    let c_did = CString::new(did).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_key_for_local_did(command_handle, wallet_handle, c_did.as_ptr(), cb);

    let response = match result_to_string(err, receiver) {
        Ok(key) => (atoms::ok(), format!("Success! {:?}", key)),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn store_their_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let identity_json: String = try!(args[1].decode());
    let c_identity_json = CString::new(identity_json).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_store_their_did(command_handle, wallet_handle, c_identity_json.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}
