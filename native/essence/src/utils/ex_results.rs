use indy::api::ErrorCode;

use std::sync::mpsc::Receiver;

pub fn result_to_empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), String> {
    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    let err = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    Ok(())
}

pub fn result_to_int(err: ErrorCode, receiver: Receiver<(ErrorCode, i32)>) -> Result<i32, i32> {
    let my_err = err as i32;
    if err != ErrorCode::Success {
        return Err(my_err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(my_err);
    }

    Ok(val)
}

pub fn result_to_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String)>) -> Result<String, i32> {
    let my_err = err as i32;
    if err != ErrorCode::Success {
        return Err(my_err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(my_err);
    }

    Ok(val)
}

pub fn result_to_string_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String, String)>) -> Result<(String, String), i32> {
    let my_err = err as i32;
    if err != ErrorCode::Success {
        return Err(my_err);
    }

    let (err, val, val2) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(my_err);
    }

    Ok((val, val2))
}
