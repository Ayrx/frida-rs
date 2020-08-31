use crate::nativepointer::NativePointer;

pub struct RangeDetails {
    pub base: NativePointer,
    pub size: usize,
    pub protection: String,
    pub file: Option<FileMapping>,
}

impl From<frida_rs_sys::range::RangeDetails> for RangeDetails {
    fn from(m: frida_rs_sys::range::RangeDetails) -> Self {
        RangeDetails {
            base: NativePointer::from_sys(m.base()),
            size: m.size(),
            protection: m.protection(),
            file: m.file().map(|s| FileMapping::from(s)),
        }
    }
}

pub struct FileMapping {
    pub path: String,
    pub offset: usize,
    pub size: usize,
}

impl From<frida_rs_sys::range::FileMapping> for FileMapping {
    fn from(m: frida_rs_sys::range::FileMapping) -> Self {
        FileMapping {
            path: m.path(),
            offset: m.offset(),
            size: m.size(),
        }
    }
}
