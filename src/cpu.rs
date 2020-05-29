//!Structures to access a CPU context.
//!
//!The structures in this module give access to the values of a CPU register
//!at various points in Frida usage. As registers are architecture dependent,
//!Frida APIs will return a [`CpuContext`](CpuContext) has to be unwrapped to
//!the appropriate struct.
use crate::nativepointer::NativePointer;
use crate::plumbing;
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

impl From<plumbing::cpu::CpuContext> for CpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        match crate::process::get_arch() {
            "ia32" => Self::Ia32CpuContext(Ia32CpuContext::from(m)),
            "x64" => Self::X64CpuContext(X64CpuContext::from(m)),
            "arm" => Self::ArmCpuContext(ArmCpuContext::from(m)),
            "arm64" => Self::Arm64CpuContext(Arm64CpuContext::from(m)),
            "mips" => Self::MipsCpuContext(MipsCpuContext::from(m)),
            _ => Self::PortableCpuContext(PortableCpuContext::from(m)),
        }
    }
}

#[derive(Debug)]
pub struct PortableCpuContext {
    pub pc: NativePointer,
    pub sp: NativePointer,
}

impl From<plumbing::cpu::CpuContext> for PortableCpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        PortableCpuContext {
            pc: NativePointer::from(Reflect::get(&m, &JsValue::from_str("pc")).unwrap()),
            sp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("sp")).unwrap()),
        }
    }
}

#[derive(Debug)]
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
}

impl From<plumbing::cpu::CpuContext> for Ia32CpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        Ia32CpuContext {
            pc: NativePointer::from(Reflect::get(&m, &JsValue::from_str("pc")).unwrap()),
            sp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("sp")).unwrap()),
            eax: NativePointer::from(Reflect::get(&m, &JsValue::from_str("eax")).unwrap()),
            ecx: NativePointer::from(Reflect::get(&m, &JsValue::from_str("ecx")).unwrap()),
            edx: NativePointer::from(Reflect::get(&m, &JsValue::from_str("edx")).unwrap()),
            ebx: NativePointer::from(Reflect::get(&m, &JsValue::from_str("ebx")).unwrap()),
            esp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("esp")).unwrap()),
            ebp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("ebp")).unwrap()),
            esi: NativePointer::from(Reflect::get(&m, &JsValue::from_str("esi")).unwrap()),
            edi: NativePointer::from(Reflect::get(&m, &JsValue::from_str("edi")).unwrap()),
            eip: NativePointer::from(Reflect::get(&m, &JsValue::from_str("eip")).unwrap()),
        }
    }
}

#[derive(Debug)]
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
}

impl From<plumbing::cpu::CpuContext> for X64CpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        X64CpuContext {
            pc: NativePointer::from(Reflect::get(&m, &JsValue::from_str("pc")).unwrap()),
            sp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("sp")).unwrap()),
            rax: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rax")).unwrap()),
            rcx: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rcx")).unwrap()),
            rdx: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rdx")).unwrap()),
            rbx: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rbx")).unwrap()),
            rsp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rsp")).unwrap()),
            rbp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rbp")).unwrap()),
            rsi: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rsi")).unwrap()),
            rdi: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rdi")).unwrap()),
            r8: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r8")).unwrap()),
            r10: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r10")).unwrap()),
            r11: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r11")).unwrap()),
            r12: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r12")).unwrap()),
            r13: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r13")).unwrap()),
            r14: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r14")).unwrap()),
            r15: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r15")).unwrap()),
            rip: NativePointer::from(Reflect::get(&m, &JsValue::from_str("rip")).unwrap()),
        }
    }
}

#[derive(Debug)]
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
}

impl From<plumbing::cpu::CpuContext> for ArmCpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        ArmCpuContext {
            pc: NativePointer::from(Reflect::get(&m, &JsValue::from_str("pc")).unwrap()),
            sp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("sp")).unwrap()),
            r0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r0")).unwrap()),
            r1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r1")).unwrap()),
            r2: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r2")).unwrap()),
            r3: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r3")).unwrap()),
            r4: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r4")).unwrap()),
            r5: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r5")).unwrap()),
            r6: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r6")).unwrap()),
            r7: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r7")).unwrap()),
            r8: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r8")).unwrap()),
            r9: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r9")).unwrap()),
            r10: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r10")).unwrap()),
            r11: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r11")).unwrap()),
            r12: NativePointer::from(Reflect::get(&m, &JsValue::from_str("r12")).unwrap()),
            lr: NativePointer::from(Reflect::get(&m, &JsValue::from_str("lr")).unwrap()),
        }
    }
}

#[derive(Debug)]
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
}

impl From<plumbing::cpu::CpuContext> for Arm64CpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        Arm64CpuContext {
            pc: NativePointer::from(Reflect::get(&m, &JsValue::from_str("pc")).unwrap()),
            sp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("sp")).unwrap()),
            x0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x0")).unwrap()),
            x1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x1")).unwrap()),
            x2: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x2")).unwrap()),
            x3: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x3")).unwrap()),
            x4: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x4")).unwrap()),
            x5: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x5")).unwrap()),
            x6: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x6")).unwrap()),
            x7: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x7")).unwrap()),
            x8: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x8")).unwrap()),
            x9: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x9")).unwrap()),
            x10: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x10")).unwrap()),
            x11: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x11")).unwrap()),
            x12: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x12")).unwrap()),
            x13: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x13")).unwrap()),
            x14: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x14")).unwrap()),
            x15: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x15")).unwrap()),
            x16: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x16")).unwrap()),
            x17: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x17")).unwrap()),
            x18: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x18")).unwrap()),
            x19: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x19")).unwrap()),
            x20: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x20")).unwrap()),
            x21: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x21")).unwrap()),
            x22: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x22")).unwrap()),
            x23: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x23")).unwrap()),
            x24: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x24")).unwrap()),
            x25: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x25")).unwrap()),
            x26: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x26")).unwrap()),
            x27: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x27")).unwrap()),
            x28: NativePointer::from(Reflect::get(&m, &JsValue::from_str("x28")).unwrap()),
            fp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("fp")).unwrap()),
            lr: NativePointer::from(Reflect::get(&m, &JsValue::from_str("lr")).unwrap()),
        }
    }
}

#[derive(Debug)]
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
}

impl From<plumbing::cpu::CpuContext> for MipsCpuContext {
    fn from(m: plumbing::cpu::CpuContext) -> Self {
        MipsCpuContext {
            pc: NativePointer::from(Reflect::get(&m, &JsValue::from_str("pc")).unwrap()),
            sp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("sp")).unwrap()),
            gp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("gp")).unwrap()),
            fp: NativePointer::from(Reflect::get(&m, &JsValue::from_str("fp")).unwrap()),
            ra: NativePointer::from(Reflect::get(&m, &JsValue::from_str("ra")).unwrap()),
            hi: NativePointer::from(Reflect::get(&m, &JsValue::from_str("hi")).unwrap()),
            lo: NativePointer::from(Reflect::get(&m, &JsValue::from_str("lo")).unwrap()),
            at: NativePointer::from(Reflect::get(&m, &JsValue::from_str("at")).unwrap()),
            v0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("v0")).unwrap()),
            v1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("v1")).unwrap()),
            a0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("a0")).unwrap()),
            a1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("a1")).unwrap()),
            a2: NativePointer::from(Reflect::get(&m, &JsValue::from_str("a2")).unwrap()),
            a3: NativePointer::from(Reflect::get(&m, &JsValue::from_str("a3")).unwrap()),
            t0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t0")).unwrap()),
            t1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t1")).unwrap()),
            t2: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t2")).unwrap()),
            t3: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t3")).unwrap()),
            t4: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t4")).unwrap()),
            t5: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t5")).unwrap()),
            t6: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t6")).unwrap()),
            t7: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t7")).unwrap()),
            t8: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t8")).unwrap()),
            t9: NativePointer::from(Reflect::get(&m, &JsValue::from_str("t9")).unwrap()),
            s0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s0")).unwrap()),
            s1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s1")).unwrap()),
            s2: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s2")).unwrap()),
            s3: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s3")).unwrap()),
            s4: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s4")).unwrap()),
            s5: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s5")).unwrap()),
            s6: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s6")).unwrap()),
            s7: NativePointer::from(Reflect::get(&m, &JsValue::from_str("s7")).unwrap()),
            k0: NativePointer::from(Reflect::get(&m, &JsValue::from_str("k0")).unwrap()),
            k1: NativePointer::from(Reflect::get(&m, &JsValue::from_str("k1")).unwrap()),
        }
    }
}
