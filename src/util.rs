use crate::error::*;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn sha256(messages: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for m in messages.iter() {
        hasher.update(m);
    }
    let result = hasher.finalize();
    let mut secret = [0u8; 32];
    secret.copy_from_slice(&result);
    secret
}
pub fn read_password_tty(q: &str, verify: bool) -> Fido2LuksResult<String> {
    read_password(q, verify, true)
}
pub fn read_password(q: &str, verify: bool, tty: bool) -> Fido2LuksResult<String> {
    let res = if tty {
        rpassword::read_password_from_tty(Some(&[q, ": "].join("")))
    } else {
        print!("{}: ", q);
        rpassword::read_password()
    }?;
    match res {
        ref pass
            if verify
                && &rpassword::read_password_from_tty(Some(&[q, "(again): "].join(" ")))?
                    != pass =>
        {
            Err(Fido2LuksError::AskPassError {
                cause: AskPassError::Mismatch,
            })
        }
        pass => Ok(pass),
    }
}

pub fn read_keyfile<P: Into<PathBuf>>(path: P) -> Fido2LuksResult<Vec<u8>> {
    let mut file = File::open(path.into())?;
    let mut key = Vec::new();
    file.read_to_end(&mut key)?;
    Ok(key)
}
