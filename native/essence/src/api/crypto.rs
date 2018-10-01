use std::ffi::CString;
use bytes::{Bytes,BytesMut,BufMut,Buf};
use rustler::{Env, Term, NifResult, Encoder};
use utils::ex_results::{result_to_string,
result_to_empty,result_to_msg_msglen,result_to_bool,result_to_sender_msg_msglen};
use utils::callbacks;
use indy::api::crypto::{indy_create_key,
    indy_set_key_metadata,
    indy_get_key_metadata,
    indy_crypto_sign,
    indy_crypto_verify,
    indy_crypto_auth_crypt,
    indy_crypto_auth_decrypt,
    indy_crypto_anon_crypt,
    indy_crypto_anon_decrypt};

pub fn create_key<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());
    let key_json: String = try!(args[1].decode()); 
    let c_key_json = CString::new(key_json).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_create_key(command_handle, wallet_handle,c_key_json.as_ptr(), cb);

    let response = result_to_string(err, receiver);

    Ok(response.encode(env))
}

pub fn set_key_metadata<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());    
    let verkey: String = try!(args[1].decode()); 
    let c_verkey = CString::new(verkey).unwrap();
    let metadata: String = try!(args[2].decode()); 
    let c_metadata = CString::new(metadata).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_set_key_metadata(command_handle, wallet_handle,c_verkey.as_ptr(),c_metadata.as_ptr(), cb);

    let response = result_to_empty(err, receiver);

    Ok(response.encode(env))
}

pub fn get_key_metadata<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());    
    let verkey: String = try!(args[1].decode()); 
    let c_verkey = CString::new(verkey).unwrap();

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_string();

    let err = indy_get_key_metadata(command_handle, wallet_handle,c_verkey.as_ptr(), cb);

    let response = result_to_string(err, receiver);

    Ok(response.encode(env))
}

pub fn crypto_sign<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());    
    let signer_vk: String = try!(args[1].decode()); 
    let c_signer_vk = CString::new(signer_vk).unwrap();
    let mut message_raw = Bytes::from(try!(args[1].decode()));
    //let message_raw: String = try!(args[2].decode()); 
    //let c_message_raw = CStr::from_bytes_with_nul(message_raw).unwrap();
    let message_len: u32 = try!(args[3].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bytes_u32();

    let err = indy_crypto_sign(command_handle, wallet_handle,c_signer_vk.as_ptr(),message_raw.as_ptr(),message_len, cb);

    let response = result_to_msg_msglen(err, receiver);

    Ok(response.encode(env))
}

pub fn crypto_verify<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let signer_vk: String = try!(args[0].decode()); 
    let c_signer_vk = CString::new(signer_vk).unwrap();
    let mut message_raw = BytesMut::with_capacity(64);
    message_raw.put(try!(args[1].decode()));
    //let message_raw: String = try!(args[1].decode()); 
    //let c_message_raw = CStr::from_bytes_with_nul(message_raw).unwrap();
    let message_len: u32 = try!(args[2].decode());
    let mut signature_raw = BytesMut::with_capacity(64);
    signature_raw.put(try!(args[3].decode()));
    //let signature_raw: String = try!(args[3].decode()); 
    //let c_signature_raw = CStr::from_bytes_with_nul(signature_raw).unwrap();
    let signature_len: u32 = try!(args[4].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bool();

    let err = indy_crypto_verify(command_handle, c_signer_vk.as_ptr(),message_raw.as_ptr(),message_len,signature_raw.as_ptr(),signature_len, cb);

    let response = result_to_bool(err, receiver); 

    Ok(response.encode(env))
}

pub fn crypto_auth_crypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());    
    let sender_vk: String = try!(args[1].decode()); 
    let c_sender_vk = CString::new(sender_vk).unwrap();
    let recipient_vk: String = try!(args[2].decode()); 
    let c_recipient_vk = CString::new(recipient_vk).unwrap();
    let mut msg_data = BytesMut::with_capacity(64);
    msg_data.put(try!(args[3].decode()));
    //let msg_data: String = try!(args[3].decode()); 
    //let c_msg_data = CStr::from_bytes_with_nul(msg_data).unwrap();
    let msg_len: u32 = try!(args[4].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bytes_u32();

    let err =
    indy_crypto_auth_crypt(command_handle,wallet_handle,c_sender_vk.as_ptr(),c_recipient_vk.as_ptr(),msg_data.as_ptr(), msg_len, cb);

    /* [encrypted_msg: :u8, encrypted_len: :u32] */
    let response = result_to_msg_msglen(err, receiver);

    Ok(response.encode(env))
}

pub fn crypto_auth_decrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());    
    let recipient_vk: String = try!(args[1].decode()); 
    let c_recipient_vk = CString::new(recipient_vk).unwrap();
    let mut encrypted_msg = BytesMut::with_capacity(64);
    encrypted_msg.put(try!(args[2].decode()));
    //let encrypted_msg: String = try!(args[2].decode()); 
    //let c_encrypted_msg = CStr::from_bytes_with_nul(encrypted_msg).unwrap();
    let encrypted_len: u32 = try!(args[3].decode());

    /* [sender_vk: :char, msg_data: :u8, msg_len: :u32] */
    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_message();


    let err = indy_crypto_auth_decrypt(command_handle, wallet_handle,c_recipient_vk.as_ptr(),encrypted_msg.as_ptr(),encrypted_len, cb);

    /* [sender_vk: :char, msg_data: :u8, msg_len: :u32] */
    let response = result_to_sender_msg_msglen(err, receiver);

    Ok(response.encode(env))
}

pub fn crypto_anon_crypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let recipient_vk: String = try!(args[0].decode()); 
    let c_recipient_vk = CString::new(recipient_vk).unwrap();
    let mut msg_data = BytesMut::with_capacity(64);
    msg_data.put(try!(args[1].decode()));
    //let msg_data = OwnedBinary.from_raw(args[1]); //try!(args[1].decode()); 
    //let c_msg_data = CStr::from_bytes_with_nul(msg_data).unwrap();
    let msg_len: u32 = try!(args[2].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bytes_u32();

    let err = indy_crypto_anon_crypt(command_handle, c_recipient_vk.as_ptr(),c_msg_data.as_ptr(),msg_len, cb);

    /* [encrypted_msg: :u8, encrypted_len: :u32] */
    let response = result_to_msg_msglen(err, receiver);

    Ok(response.encode(env))
}

pub fn crypto_anon_decrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let wallet_handle: i32 = try!(args[0].decode());    
    let recipient_vk: String = try!(args[1].decode()); 
    let c_recipient_vk = CString::new(recipient_vk).unwrap();
    //let encrypted_msg = Binary.from_term(args[2]); 
    let mut encrypted_msg = BytesMut::with_capacity(64);
    encrypted_msg.put(try!(args[2].decode()));
    //let c_encrypted_msg = CStr::from_bytes_with_nul(encrypted_msg).unwrap();
    let encrypted_len: u32 = try!(args[3].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec_bytes_u32();

    let err = indy_crypto_anon_decrypt(command_handle, wallet_handle,c_recipient_vk.as_ptr(),encrypted_msg.as_ptr(),encrypted_len, cb);

    /* [msg_data: :u8, msg_len: :u32] */
    let response = result_to_msg_msglen(err, receiver);

    Ok(response.encode(env))
}
