use std::fmt;

use bytes::Bytes;
use tlv::Tlv;
use tlv::VarNumber;

#[derive(Debug, PartialEq)]
pub struct MustBeFresh;

impl Tlv for MustBeFresh {
    const TYPE: u64 = 0x12;

    fn length(&self) -> VarNumber {
        0u64.into()
    }

    fn value(&self) -> Option<Bytes> {
        None
    }

    fn size(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Display for MustBeFresh {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "MustBeFresh")
    }
}