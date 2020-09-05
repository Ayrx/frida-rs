//!Structures to access a CPU context.
//!
//!The structures in this module give access to the values of a CPU register
//!at various points in Frida usage. As registers are architecture dependent,
//!Frida APIs will return a [`CpuContext`](CpuContext) has to be unwrapped to
//!the appropriate struct.
use crate::fromsys::FromSys;
use crate::nativepointer::NativePointer;
use frida_rs_derive::*;
use frida_rs_sys::cpu;
use js_sys::Reflect;
use wasm_bindgen::prelude::JsValue;

#[derive(Debug)]
pub enum CpuContext {
    Ia32CpuContext(Ia32CpuContext),
    X64CpuContext(X64CpuContext),
    ArmCpuContext(ArmCpuContext),
    Arm64CpuContext(Arm64CpuContext),
    MipsCpuContext(MipsCpuContext),
    PortableCpuContext(PortableCpuContext),
}

impl FromSys<cpu::CpuContext> for CpuContext {
    fn from_sys(m: cpu::CpuContext) -> Self {
        match crate::process::get_arch() {
            "ia32" => Self::Ia32CpuContext(Ia32CpuContext::from_sys(m)),
            "x64" => Self::X64CpuContext(X64CpuContext::from_sys(m)),
            "arm" => Self::ArmCpuContext(ArmCpuContext::from_sys(m)),
            "arm64" => Self::Arm64CpuContext(Arm64CpuContext::from_sys(m)),
            "mips" => Self::MipsCpuContext(MipsCpuContext::from_sys(m)),
            _ => Self::PortableCpuContext(PortableCpuContext::from_sys(m)),
        }
    }

    fn into_sys(self) -> cpu::CpuContext {
        match self {
            Self::Ia32CpuContext(c) => c.into_sys(),
            Self::X64CpuContext(c) => c.into_sys(),
            Self::ArmCpuContext(c) => c.into_sys(),
            Self::Arm64CpuContext(c) => c.into_sys(),
            Self::MipsCpuContext(c) => c.into_sys(),
            Self::PortableCpuContext(c) => c.into_sys(),
        }
    }
}

#[derive(Debug, DeriveCpu)]
pub struct PortableCpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
    sys: cpu::CpuContext,
}

#[derive(Debug, DeriveCpu)]
pub struct Ia32CpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
    pub eax: NativePointer,
    pub ecx: NativePointer,
    pub edx: NativePointer,
    pub ebx: NativePointer,
    pub esp: NativePointer,
    pub ebp: NativePointer,
    pub esi: NativePointer,
    pub edi: NativePointer,
    pub eip: NativePointer,
    sys: cpu::CpuContext,
}

#[derive(Debug, DeriveCpu)]
pub struct X64CpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
    pub rax: NativePointer,
    pub rcx: NativePointer,
    pub rdx: NativePointer,
    pub rbx: NativePointer,
    pub rsp: NativePointer,
    pub rbp: NativePointer,
    pub rsi: NativePointer,
    pub rdi: NativePointer,
    pub r8: NativePointer,
    pub r10: NativePointer,
    pub r11: NativePointer,
    pub r12: NativePointer,
    pub r13: NativePointer,
    pub r14: NativePointer,
    pub r15: NativePointer,
    pub rip: NativePointer,
    sys: cpu::CpuContext,
}

#[derive(Debug, DeriveCpu)]
pub struct ArmCpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
    pub r0: NativePointer,
    pub r1: NativePointer,
    pub r2: NativePointer,
    pub r3: NativePointer,
    pub r4: NativePointer,
    pub r5: NativePointer,
    pub r6: NativePointer,
    pub r7: NativePointer,
    pub r8: NativePointer,
    pub r9: NativePointer,
    pub r10: NativePointer,
    pub r11: NativePointer,
    pub r12: NativePointer,
    pub lr: NativePointer,
    sys: cpu::CpuContext,
}

#[derive(Debug, DeriveCpu)]
pub struct Arm64CpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
    pub x0: NativePointer,
    pub x1: NativePointer,
    pub x2: NativePointer,
    pub x3: NativePointer,
    pub x4: NativePointer,
    pub x5: NativePointer,
    pub x6: NativePointer,
    pub x7: NativePointer,
    pub x8: NativePointer,
    pub x9: NativePointer,
    pub x10: NativePointer,
    pub x11: NativePointer,
    pub x12: NativePointer,
    pub x13: NativePointer,
    pub x14: NativePointer,
    pub x15: NativePointer,
    pub x16: NativePointer,
    pub x17: NativePointer,
    pub x18: NativePointer,
    pub x19: NativePointer,
    pub x20: NativePointer,
    pub x21: NativePointer,
    pub x22: NativePointer,
    pub x23: NativePointer,
    pub x24: NativePointer,
    pub x25: NativePointer,
    pub x26: NativePointer,
    pub x27: NativePointer,
    pub x28: NativePointer,
    pub fp: NativePointer,
    pub lr: NativePointer,
    sys: cpu::CpuContext,
}

#[derive(Debug, DeriveCpu)]
pub struct MipsCpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
    pub gp: NativePointer,
    pub fp: NativePointer,
    pub ra: NativePointer,
    pub hi: NativePointer,
    pub lo: NativePointer,
    pub at: NativePointer,
    pub v0: NativePointer,
    pub v1: NativePointer,
    pub a0: NativePointer,
    pub a1: NativePointer,
    pub a2: NativePointer,
    pub a3: NativePointer,
    pub t0: NativePointer,
    pub t1: NativePointer,
    pub t2: NativePointer,
    pub t3: NativePointer,
    pub t4: NativePointer,
    pub t5: NativePointer,
    pub t6: NativePointer,
    pub t7: NativePointer,
    pub t8: NativePointer,
    pub t9: NativePointer,
    pub s0: NativePointer,
    pub s1: NativePointer,
    pub s2: NativePointer,
    pub s3: NativePointer,
    pub s4: NativePointer,
    pub s5: NativePointer,
    pub s6: NativePointer,
    pub s7: NativePointer,
    pub k0: NativePointer,
    pub k1: NativePointer,
    sys: cpu::CpuContext,
}
