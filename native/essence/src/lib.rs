#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;

extern crate libc;

extern crate indy;

use rustler::{Env, Term, NifResult, Encoder};
use std::sync::mpsc::{channel, Receiver};
use std::ffi::CStr;
use std::collections::HashMap;
use std::sync::Mutex;
use libc::c_char;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.Indy",
    [
        ("list_pools", 0, list_pools),

    ],
    None
}


//mod pool {
    fn list_pools<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

        let (receiver, command_handle, cb) = _closure_to_cb_ec_string();

        let err = unsafe { indy::api::pool::indy_list_pools(command_handle, cb) };

        let response = match result_to_string(err, receiver) {
            Ok(resp) => resp,
            _ => String::new(),
        };

        Ok((atoms::ok(), response).encode(env))
    }
//}

fn result_to_string(err: indy::api::ErrorCode, receiver: Receiver<(indy::api::ErrorCode, String)>) -> Result<String, indy::api::ErrorCode> {
    if err != indy::api::ErrorCode::Success {
        return Err(err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != indy::api::ErrorCode::Success {
        return Err(err);
    }

    Ok(val)
}

fn _closure_to_cb_ec_string() -> (Receiver<(indy::api::ErrorCode, String)>, i32,
                                      Option<extern fn(command_handle: i32,
                                                       err: indy::api::ErrorCode,
                                                       c_str: *const c_char)>) {
    let (sender, receiver) = channel();

    lazy_static! {
        static ref CALLBACKS: Mutex < HashMap < i32, Box < FnMut(indy::api::ErrorCode, String) + Send > >> = Default::default();
    }

    let closure = Box::new(move |err, val| {
        sender.send((err, val)).unwrap();
    });

    extern "C" fn _callback(command_handle: i32, err: indy::api::ErrorCode, c_str: *const c_char) {
        let mut callbacks = CALLBACKS.lock().unwrap();
        let mut cb = callbacks.remove(&command_handle).unwrap();
        let metadata = unsafe { CStr::from_ptr(c_str).to_str().unwrap().to_string() };
        cb(err, metadata)
    }

    let mut callbacks = CALLBACKS.lock().unwrap();
    let command_handle = SequenceUtils::get_next_id();
    callbacks.insert(command_handle, closure);

    (receiver, command_handle, Some(_callback))
}

pub type IndyHandle = i32;


pub struct SequenceUtils {}

lazy_static! {
    static ref IDS_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT; //TODO use AtomicI32
}

impl SequenceUtils {
    pub fn get_next_id() -> i32 {
        (IDS_COUNTER.fetch_add(1, Ordering::SeqCst) + 1) as i32
    }
}
