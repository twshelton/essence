#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;

extern crate libc;
extern crate indy;

mod callbacks;
mod utils;
mod results;

use libc::c_char;
use std::ffi::CString;
use indy::api::ErrorCode;
use rustler::{Env, Term, NifResult, Encoder};
use results::{result_to_string, result_to_int, result_to_empty};
use
indy::api::wallet::{indy_create_wallet,indy_open_wallet,indy_close_wallet,indy_delete_wallet,indy_export_wallet,indy_import_wallet};

mod atoms {
   rustler_atoms! {
       atom ok;
       atom error;
       //atom __true__ = "true";
       //atom __false__ = "false";
   }
}

rustler_export_nifs! {
   "Elixir.Indy.Wallett" ,
    [
        ("import_wallet", 3, import_wallet),
        ("export_wallet", 2, export_wallet),
        ("delete_wallet", 2, delete_wallet),
        ("close_wallet", 1, close_wallet),
        ("open_wallet", 2, open_wallet),
        ("create_wallet", 2, create_wallet),
    ],
   None
}

pub fn create_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let credentials: String = try!(args[1].decode());
    let c_config = CString::new(config).unwrap();
    let c_credentials = CString::new(credentials).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_create_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem creating the wallet: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn open_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let credentials: String = try!(args[1].decode());
    let c_config = CString::new(config).unwrap();
    let c_credentials = CString::new(credentials).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_i32();

    let err = indy_open_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), cb);

    let response = match result_to_int(err, receiver) {
        Ok(handle) => (atoms::ok(), handle),
        Err(_err) => (atoms::error(), 0),
    };

    Ok(response.encode(env))
}

pub fn close_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

  let handle: i32 = try!(args[0].decode());

  let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

  let err = indy_close_wallet(command_handle, handle, cb);

  let response = match result_to_empty(err, receiver) {
      Ok(()) => (atoms::ok(), format!("Success!")),
      Err(err) => (atoms::error(), format!("There was a problemi closing the wallet: {:?}", err)),
  };

  Ok(response.encode(env))
}

pub fn delete_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let credentials: String = try!(args[1].decode());
    let c_config = CString::new(config).unwrap();
    let c_credentials = CString::new(credentials).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_delete_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn export_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let handle: i32 = try!(args[0].decode());
    let config: String = try!(args[1].decode());
    let c_config = CString::new(config).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_export_wallet(command_handle, handle, c_config.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem exporting this wallet: {:?}", err)),
    };

    Ok(response.encode(env))
}

pub fn import_wallet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config: String = try!(args[0].decode());
    let c_config = CString::new(config).unwrap();
    let credentials: String = try!(args[1].decode());
    let c_credentials = CString::new(credentials).unwrap();
    let import_config: String = try!(args[2].decode());
    let c_import_config = CString::new(import_config).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_import_wallet(command_handle, c_config.as_ptr(), c_credentials.as_ptr(), c_import_config.as_ptr(), cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem importing this wallet: {:?}", err)),
    };

    Ok(response.encode(env))
}
