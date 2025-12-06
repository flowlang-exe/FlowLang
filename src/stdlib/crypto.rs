use crate::error::FlowError;
use crate::types::{NativeFn, Value};
use std::sync::Arc;
use sha2::{Sha256, Sha512, Digest};
use md5::Md5;

pub fn load_crypto_module() -> Vec<(&'static str, Value)> {
    vec![
        // Hashing
        ("md5", Value::NativeFunction(NativeFn::new(crypto_md5))),
        ("sha256", Value::NativeFunction(NativeFn::new(crypto_sha256))),
        ("sha512", Value::NativeFunction(NativeFn::new(crypto_sha512))),
        
        // Encoding
        ("base64_encode", Value::NativeFunction(NativeFn::new(crypto_base64_encode))),
        ("base64_decode", Value::NativeFunction(NativeFn::new(crypto_base64_decode))),
        
        // Hex encoding
        ("hex_encode", Value::NativeFunction(NativeFn::new(crypto_hex_encode))),
        ("hex_decode", Value::NativeFunction(NativeFn::new(crypto_hex_decode))),
    ]
}

// MD5 hash
fn crypto_md5(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::md5 expects 1 argument (text)",
            0,
            0,
        ));
    }

    let text = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::md5 expects a Silk (string)", 0, 0)),
    };

    let mut hasher = Md5::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::String(Arc::new(hash)))
}

// SHA256 hash
fn crypto_sha256(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::sha256 expects 1 argument (text)",
            0,
            0,
        ));
    }

    let text = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::sha256 expects a Silk (string)", 0, 0)),
    };

    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::String(Arc::new(hash)))
}

// SHA512 hash
fn crypto_sha512(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::sha512 expects 1 argument (text)",
            0,
            0,
        ));
    }

    let text = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::sha512 expects a Silk (string)", 0, 0)),
    };

    let mut hasher = Sha512::new();
    hasher.update(text.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    Ok(Value::String(Arc::new(hash)))
}

// Base64 encode
fn crypto_base64_encode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::base64_encode expects 1 argument (text)",
            0,
            0,
        ));
    }

    let text = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::base64_encode expects a Silk (string)", 0, 0)),
    };

    use base64::{Engine as _, engine::general_purpose};
    let encoded = general_purpose::STANDARD.encode(text.as_bytes());

    Ok(Value::String(Arc::new(encoded)))
}

// Base64 decode
fn crypto_base64_decode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::base64_decode expects 1 argument (encoded text)",
            0,
            0,
        ));
    }

    let encoded = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::base64_decode expects a Silk (string)", 0, 0)),
    };

    use base64::{Engine as _, engine::general_purpose};
    match general_purpose::STANDARD.decode(encoded.as_bytes()) {
        Ok(decoded) => {
            match String::from_utf8(decoded) {
                Ok(text) => Ok(Value::String(Arc::new(text))),
                Err(_) => Err(FlowError::runtime("Invalid UTF-8 in decoded data", 0, 0)),
            }
        }
        Err(_) => Err(FlowError::runtime("Invalid base64 string", 0, 0)),
    }
}

// Hex encode
fn crypto_hex_encode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::hex_encode expects 1 argument (text)",
            0,
            0,
        ));
    }

    let text = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::hex_encode expects a Silk (string)", 0, 0)),
    };

    let hex = text.as_bytes().iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    Ok(Value::String(Arc::new(hex)))
}

// Hex decode
fn crypto_hex_decode(args: Vec<Value>) -> Result<Value, FlowError> {
    if args.len() != 1 {
        return Err(FlowError::runtime(
            "crypto::hex_decode expects 1 argument (hex string)",
            0,
            0,
        ));
    }

    let hex = match &args[0] {
        Value::String(s) => s.clone(),
        _ => return Err(FlowError::type_error("crypto::hex_decode expects a Silk (string)", 0, 0)),
    };

    let bytes: Result<Vec<u8>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect();

    match bytes {
        Ok(b) => {
            match String::from_utf8(b) {
                Ok(text) => Ok(Value::String(Arc::new(text))),
                Err(_) => Err(FlowError::runtime("Invalid UTF-8 in decoded data", 0, 0)),
            }
        }
        Err(_) => Err(FlowError::runtime("Invalid hex string", 0, 0)),
    }
}
