
extern crate rkdb;

use std::ffi;
use rkdb::{
  api,
  kapi,
  kbindings,
  kbindings::*
};

fn main() {

  let host = "127.0.0.1";
  let username = "username";
  let port = 6000;
  let chost = ffi::CString::new(host).unwrap();
  let cuser = ffi::CString::new(username).unwrap();
  let handle = match unsafe { kapi::khpu(chost.as_ptr(), port, cuser.as_ptr()) } {
    h if h < 0 => { println!("{}", "ERROR: Connection failure".to_string()); std::process::exit(1); },
    0 => { println!("{}", "ERROR: Bad credentials".to_string()); std::process::exit(1); },
    h => h
  };

  let query = ".u.upd";
  let cquery = ffi::CString::new(query).unwrap();
  let cnum = KVal::new(serial(klong(64)));
  let cnum2 = KVal::new(serial(klong(128)));
  let cnums:[KVal;2] = [cnum, cnum2];
  let clist = kmixed(&cnums);
  let kptr = unsafe { kapi::k(handle, cquery.as_ptr(), clist, kvoid()) };
  if kptr.is_null() {
    println!("{}", "ERROR: Query failed".to_string());
    std::process::exit(1);
  }
  println!("{}", "Dispatched!".to_string());

  unsafe { kapi::kclose(handle) };
}

