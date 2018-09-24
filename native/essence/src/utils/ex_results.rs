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

pub fn result_to_int(err: ErrorCode, receiver: Receiver<(ErrorCode, i32)>) -> Result<i32, String> {
    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    Ok(val)
}

pub fn result_to_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String)>) -> Result<String, String> {
    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    Ok(val)
}

pub fn result_to_string_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String, String)>) -> Result<(String,String), String> {
    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    let (err, val, val2) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    Ok((val, val2))
}
pub fn result_to_string_string_timestamp(err: ErrorCode, receiver: Receiver<(ErrorCode, String, String, u64)>) -> Result<(String, String, u64), String> {
    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    let (err, val, val2, ts) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(format!("{:?}", err));
    }

    Ok((val, val2, ts))
}
