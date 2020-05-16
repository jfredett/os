#![cfg(test)]
use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11
}

/// The port number set in Cargo.toml as 'iobase'
static ISA_DEBUG_PORT: u16 = 0xf4;

pub fn exit_qemu(code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(ISA_DEBUG_PORT);
        port.write(code as u32);
    }
}


