use crate::cpu::CpuContext;
use crate::plumbing;
use std::fmt;
use std::str;
use std::str::FromStr;

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

impl From<plumbing::thread::ThreadDetails> for ThreadDetails {
    fn from(m: plumbing::thread::ThreadDetails) -> Self {
        ThreadDetails {
            id: m.id(),
            state: ThreadState::from_str(&m.state()).unwrap(),
            context: CpuContext::from(m.context()),
        }
    }
}
