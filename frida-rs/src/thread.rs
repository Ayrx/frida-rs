//!Frida functions for thread-level functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#thread](https://frida.re/docs/javascript-api/#thread)

use crate::cpu::CpuContext;
use crate::fromsys::FromSys;
use crate::NativePointer;
use std::fmt;
use std::str;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

pub enum ThreadState {
    Running,
    Stopped,
    Waiting,
    Uninterruptible,
    Halted,
}

impl str::FromStr for ThreadState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "running" => Ok(ThreadState::Running),
            "stopped" => Ok(ThreadState::Stopped),
            "waiting" => Ok(ThreadState::Waiting),
            "uninterruptible" => Ok(ThreadState::Uninterruptible),
            "halted" => Ok(ThreadState::Halted),
            _ => Err(format!("'{}' is not a valid value for ThreadState", s)),
        }
    }
}

impl fmt::Display for ThreadState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ThreadState::Running => "Running",
            ThreadState::Stopped => "Stopped",
            ThreadState::Waiting => "Waiting",
            ThreadState::Uninterruptible => "Uninterruptible",
            ThreadState::Halted => "Halted",
        };
        write!(f, "{}", printable)
    }
}

pub struct ThreadDetails {
    pub id: u32,
    pub state: ThreadState,
    pub context: CpuContext,
}

impl From<frida_rs_sys::thread::ThreadDetails> for ThreadDetails {
    fn from(m: frida_rs_sys::thread::ThreadDetails) -> Self {
        ThreadDetails {
            id: m.id(),
            state: ThreadState::from_str(&m.state()).unwrap(),
            context: CpuContext::from_sys(m.context()),
        }
    }
}

///Suspend the execution of the current thread for `delay` seconds.
///
///This is equivalent to calling `Thread.sleep` in the JavaScript API.
pub fn sleep(delay: f64) {
    frida_rs_sys::thread::sleep(JsValue::from_f64(delay));
}

pub fn backtrace(ctx: Option<CpuContext>) -> Vec<NativePointer> {
    let mut ret = Vec::new();

    let c = ctx.map(|s| s.into_sys());
    let bt = frida_rs_sys::thread::backtrace(c);

    for i in bt.iter() {
        ret.push(NativePointer::from_jsvalue(i));
    }

    ret
}
