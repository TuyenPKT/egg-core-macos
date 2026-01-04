use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn eggcore_start_miner(threads: u32) {
    crate::miner::start(threads);
}

#[no_mangle]
pub extern "C" fn eggcore_stop_miner() {
    crate::miner::stop();
}

#[no_mangle]
pub extern "C" fn eggcore_wallet_create(path: *const c_char, pass: *const c_char) {
    let path = unsafe { CStr::from_ptr(path) }.to_str().unwrap();
    let pass = unsafe { CStr::from_ptr(pass) }.to_str().unwrap();

    crate::wallet::create(path, pass);
}
