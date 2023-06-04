use crate::error::{AllocateReason, OldPciError, OldPciResult};

pub struct AlignedAddress(u64);

impl AlignedAddress {
    pub fn new_uncheck(addr: u64) -> Self {
        Self(addr)
    }

    pub fn new_with_check_align_64_bytes(addr: u64) -> OldPciResult<Self> {
        if is_align_64_bytes(addr) {
            Ok(Self::new_uncheck(addr))
        } else {
            Err(OldPciError::FailedAllocate(
                AllocateReason::NotAlignedAddress {
                    expect_align_size: 64,
                },
            ))
        }
    }

    pub fn address(&self) -> OldPciResult<u64> {
        if is_align_64_bytes(self.0) {
            Ok(self.0)
        } else {
            Err(OldPciError::FailedAllocate(
                AllocateReason::NotAlignedAddress {
                    expect_align_size: 64,
                },
            ))
        }
    }
}

fn is_align_64_bytes(value: u64) -> bool {
    (value % 64) == 0
}
