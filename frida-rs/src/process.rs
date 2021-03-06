//!Frida functions for process-level functionality.
//!
//!The functions in this module correspond to the JavaScript functions
//!grouped under
//![https://frida.re/docs/javascript-api/#process](https://frida.re/docs/javascript-api/#process).

use crate::fromsys::FromSys;
use crate::module;
use crate::nativepointer;
use crate::range::RangeDetails;
use crate::thread;
use frida_rs_sys::process;

///Get the PID of the instrumented process.
///
///This is equivalent to calling `Process.id` in the JavaScript API.
pub fn get_pid() -> u32 {
    *process::id
}

///Get the architecture of the instrumented process.
///
///This is equivalent to calling `Process.arch` in the JavaScript API.
pub fn get_arch() -> &'static str {
    &*process::arch
}

///Get the platform of the instrumented process.
///
///This is equivalent to calling `Process.platform` in the JavaScript API.
pub fn get_platform() -> &'static str {
    &*process::platform
}

///Get the page size of the instrumented process.
///
///This is equivalent to calling `Process.pageSize` in the JavaScript API.
pub fn get_page_size() -> usize {
    *process::page_size
}

///Get the pointer size of the instrumented process.
///
///This is equivalent to calling `Process.pointerSize` in the JavaScript API.
pub fn get_pointer_size() -> usize {
    *process::pointer_size
}

///Get the code signing policy of the instrumented process.
///
///This is equivalent to calling `Process.codeSigningPolicy` in the JavaScript
///API.
pub fn get_code_signing_policy() -> &'static str {
    &*process::code_signing_policy
}

///Check if a debugger is attached to the instrumented process.
///
///This is equivalent to calling `Process.isDebuggerAttached()` in the
///JavaScript API.
pub fn is_debugger_attached() -> bool {
    process::is_debugger_attached()
}

///Get the TID of the current thread.
///
///This is equivalent to calling `Process.getCurrentThreadId()` in the
///JavaScript API.
pub fn get_tid() -> u32 {
    process::get_current_thread_id()
}

///Get all threads in the instrumented process.
///
///This is the equivalent to calling `Process.enumerateThreads()` in the
///JavaScript API.
pub fn enumerate_threads() -> Vec<thread::ThreadDetails> {
    frida_rs_sys::process::enumerate_threads()
        .iter()
        .map(frida_rs_sys::thread::ThreadDetails::from)
        .map(crate::thread::ThreadDetails::from)
        .collect()
}

///Get all loaded modules in the instrumented process.
///
///This is the equivalent to calling `Process.enumerateModules()` in the
///JavaScript API.
pub fn enumerate_modules() -> Vec<module::Module> {
    frida_rs_sys::process::enumerate_modules()
        .iter()
        .map(frida_rs_sys::module::Module::from)
        .map(crate::module::Module::from_sys)
        .collect()
}

///Get a module by name.
///
///This is the equivalent to calling `Process.findModuleByName()` /
///`Process.getModuleByName()` in the JavaScript API.
pub fn get_module_by_name(name: &str) -> Option<module::Module> {
    let ret = process::get_module_by_name(name);

    if ret.is_null() {
        return None;
    }

    Some(module::Module::from_sys(ret))
}

///Get a module by address.
///
///This is the equivalent to calling `Process.findModuleByAddress()` /
///`Process.getModuleByAddress()` in the JavaScript API.
pub fn get_module_by_address(address: &nativepointer::NativePointer) -> Option<module::Module> {
    let ret = process::get_module_by_address(&address.to_sys());

    if ret.is_null() {
        return None;
    }

    Some(module::Module::from_sys(ret))
}

///Get memory range containing `address`.
///
///This is the equivalent to calling `Process.findRangeByAddress()` /
///`Process.getRangeByAddress()` in the JavaScript API.
pub fn get_range_by_address(address: &nativepointer::NativePointer) -> Option<RangeDetails> {
    let ret = process::get_range_by_address(&address.to_sys());

    if ret.is_null() {
        return None;
    }

    Some(RangeDetails::from(frida_rs_sys::range::RangeDetails::from(
        ret,
    )))
}

///Get all memory ranges satisfying `protection`.
///
///`protection` is a string with the form "rwx" where "rw-" means "must be
///at least readable and writable."
///
///This is the equivalent to calling `Process.enumerateRanges()` in the
///JavaScript API.
pub fn enumerate_ranges(protection: &str) -> Vec<RangeDetails> {
    frida_rs_sys::process::enumerate_ranges(protection)
        .iter()
        .map(frida_rs_sys::range::RangeDetails::from)
        .map(crate::range::RangeDetails::from)
        .collect()
}

///Get all individual memory allocations known to the system heap.
///
///This is the equivalent to calling `Process.enumerateMallocRanges()` in the
///JavaScript API.
pub fn enumerate_malloc_ranges() -> Vec<RangeDetails> {
    frida_rs_sys::process::enumerate_malloc_ranges()
        .iter()
        .map(frida_rs_sys::range::RangeDetails::from)
        .map(crate::range::RangeDetails::from)
        .collect()
}
