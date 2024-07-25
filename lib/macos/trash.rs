extern crate libc;

use objc2_foundation::{NSFileManager, NSString, NSURL};

use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };

    let ns_path = NSString::from_str(target);
    let manager = unsafe { NSFileManager::defaultManager() };
    let url = unsafe { NSURL::fileURLWithPath(&ns_path) };

    match unsafe { manager.trashItemAtURL_resultingItemURL_error(&url, None) } {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::String(e.to_string())),
    }
}
