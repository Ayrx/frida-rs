use crate::nativepointer::NativePointer;
use crate::plumbing;

pub struct RangeDetails {
    pub base: NativePointer,
    pub size: usize,
    pub protection: String,
    pub file: Option<FileMapping>,
}

impl From<plumbing::range::RangeDetails> for RangeDetails {
    fn from(m: plumbing::range::RangeDetails) -> Self {
        RangeDetails {
            base: m.base(),
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

impl From<plumbing::range::FileMapping> for FileMapping {
    fn from(m: plumbing::range::FileMapping) -> Self {
        FileMapping {
            path: m.path(),
            offset: m.offset(),
            size: m.size(),
        }
    }
}
