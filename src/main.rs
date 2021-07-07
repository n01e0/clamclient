use std::ffi::CString;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::env;

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} file", args[0]);
        return Ok(())
    }
    let f = Path::new(&args[1]).canonicalize().unwrap();
    let mut stream = UnixStream::connect("/var/run/clamav/clamd.ctl")?;
    let payload = CString::new(format!("CONTSCAN {}", f.display())).unwrap();
    
    stream.write_all(payload.as_bytes())?;

    let mut resp = String::new();
    stream.read_to_string(&mut resp)?;
    println!("{}", resp);
    Ok(())
}
