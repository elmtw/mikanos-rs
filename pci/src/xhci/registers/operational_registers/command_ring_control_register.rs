use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

pub mod command_abort;
pub mod command_ring_pointer;
pub mod command_ring_running;
pub mod command_stop;
pub mod crcr_field;
pub mod ring_cycle_state;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CommandRingControlRegisterOffset(usize);

impl CommandRingControlRegisterOffset {
    pub fn new(offset: OperationalRegistersOffset) -> Self {
        Self(offset.offset() + 0x18)
    }

    pub fn offset(&self) -> usize {
        self.0
    }
}
