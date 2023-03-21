use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::PciResult;
use crate::wait_update_64bits_register_for;
use crate::xhci::registers::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;

/// ERSTBA
///
/// # Offset
///
/// InterrupterRegisterSetOffset + 0x10 Bytes
///
/// # Size
///
/// 64 Bits
///
/// # Attribute
/// RW
///
/// # Description
///
/// EventRingSegmentTableの先頭アドレスを保持します。
///
/// [Xhci Document] : 428 Page
///
/// [Xhci Document]: https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(VolatileBits)]
#[volatile_type(u64)]
#[add_addr_bytes(0x10)]
#[offset_bit(6)]
pub struct EventRingSegmentTableBaseAddress(usize, PhantomData<InterrupterRegisterSetOffset>);

impl EventRingSegmentTableBaseAddress {
    pub fn update_event_ring_segment_table_addr(&self, addr: usize) -> PciResult {
        let write_addr = addr as u64;
        self.write_volatile(write_addr);
        wait_update_64bits_register_for(10, write_addr, self)
    }
}
