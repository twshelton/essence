use std::ffi::CString;
use rustler::{Env, Term, NifResult, Encoder};
use utils::ex_results::{result_to_int, result_to_empty};
use indy::api::wallet::{indy_create_wallet,indy_open_wallet,indy_close_wallet,indy_delete_wallet,indy_export_wallet,indy_import_wallet};
use utils::callbacks;

pub fn create_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let config: String = try!(args[0].decode());
    let credentials: String = try!(args[1].decode());
    let c_config = CString::new(config.to_string()).unwrap();
    let c_credentials = CString::new(credentials).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_create_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn open_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let credentials: String = try!(args[1].decode());
    let c_config = CString::new(config.to_string()).unwrap();
    let c_credentials = CString::new(credentials).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_i32();

    let err = indy_open_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), cb);

    let response = result_to_int(err, receiver);

    Ok(response.encode(env))
}

pub fn close_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let handle: i32 = try!(args[0].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_close_wallet(command_handle, handle, cb);
  //TODO Check to make sure this is the currently open wallet
  let response = result_to_empty(err, receiver);

  Ok(response.encode(env))
}

pub fn delete_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let credentials: String = try!(args[1].decode());
    let c_config = CString::new(config.to_string()).unwrap();
    let c_credentials = CString::new(credentials).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_delete_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn export_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let handle: i32 = try!(args[0].decode());
    let config: String = try!(args[1].decode());
    let c_config = CString::new(config.to_string()).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_export_wallet(command_handle, handle, c_config.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn import_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let c_config = CString::new(config.to_string()).unwrap();
    let credentials: String = try!(args[1].decode());
    let c_credentials = CString::new(credentials).unwrap();
    let import_config: String = try!(args[2].decode());
    let c_import_config = CString::new(import_config.to_string()).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_import_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), c_import_config.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}
