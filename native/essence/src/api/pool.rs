
use libc::c_char;
use std::ptr;
use std::ffi::CString;
use rustler::{Env, Term, NifResult, Encoder};
use utils::ex_results::{result_to_string, result_to_int, result_to_empty};
use utils::atoms;
use utils::callbacks;
use indy::api::pool::{
    indy_close_pool_ledger, 
    indy_refresh_pool_ledger, 
    indy_create_pool_ledger_config,
    indy_delete_pool_ledger_config, 
    indy_list_pools, 
    indy_set_protocol_version,
    indy_open_pool_ledger
    };

pub fn close_pool_ledger<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let handle: i32 = try!(args[0].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_close_pool_ledger(command_handle, handle, cb);
    //TODO: Provide a way to get currently connected pool to make sure it is the one we are closing
    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn refresh_pool_ledger<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let handle: i32 = try!(args[0].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_refresh_pool_ledger(command_handle, handle, cb);

    //TODO: Provide a way to get currently connected pool to make sure it is the one we are refreshing
    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn delete_pool_ledger_config<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config_name: String = try!(args[0].decode());
    let c_config_name = CString::new(config_name.to_string()).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_delete_pool_ledger_config(command_handle, c_config_name.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn open_default_pool_ledger<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config_name: String = try!(args[0].decode());
    let c_config_name = CString::new(config_name.to_string()).unwrap();
    let c_null: *const c_char = ptr::null();
    let protocol_version: usize = 0;

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_i32();

    let err = indy_open_pool_ledger(command_handle, c_config_name.as_ptr(), c_null, cb);

    let response = result_to_int(err, receiver); 

    Ok((response).encode(env))
}

pub fn open_pool_ledger<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config_name: String = try!(args[0].decode());
    let config: String = try!(args[1].decode());
    let c_config_name = CString::new(config_name.to_string()).unwrap();
    let c_config = CString::new(config).unwrap();
    let protocol_version: usize = 0;
    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_i32();

    let err = indy_open_pool_ledger(command_handle, c_config_name.as_ptr(), c_config.as_ptr(), cb);

    let response = result_to_int(err, receiver);

    Ok((response).encode(env))
}

pub fn create_pool_ledger_config<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let config_name: String = try!(args[0].decode());
    let config_file: String = try!(args[1].decode());
    let config: String = json!({ "genesis_txn": config_file }).to_string();
    let c_config_name = CString::new(config_name.to_string()).unwrap();
    let c_config = CString::new(config).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_create_pool_ledger_config(command_handle, c_config_name.as_ptr(), c_config.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn list_pools<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_list_pools(command_handle, cb);

    let response = result_to_string(err, receiver);

    Ok((response).encode(env))

}

pub fn set_protocol_version<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let ver: usize = try!(args[0].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_set_protocol_version(command_handle, ver, cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))

}
