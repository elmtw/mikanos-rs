use crate::error::{OperationReason, PciError, PciResult};
use crate::error::OperationReason::FailedAllocate;
use crate::VolatileAccessible;
use crate::xhci::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_abort::CommandAbort;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_ring_pointer::CommandRingPointer;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use crate::xhci::registers::operational_registers::command_ring_control_register::command_stop::CommandStop;
use crate::xhci::registers::operational_registers::command_ring_control_register::crcr_field::CrcrField;
use crate::xhci::registers::operational_registers::command_ring_control_register::ring_cycle_state::RingCycleState;
use crate::xhci::registers::operational_registers::operation_registers_offset::OperationalRegistersOffset;

pub mod command_abort;
pub mod command_ring_pointer;
pub mod command_ring_running;
pub mod command_stop;
pub mod crcr_field;
pub mod ring_cycle_state;

#[derive(Debug)]
pub struct CommandRingControlRegister {
    pub rcs: RingCycleState,
    pub cs: CommandStop,
    pub ca: CommandAbort,
    pub crr: CommandRingRunning,
    pub command_ring_pointer: CommandRingPointer,
}

impl CommandRingControlRegister {
    pub fn new(offset: CommandRingControlRegisterOffset) -> PciResult<Self> {
        Ok(Self {
            rcs: RingCycleState::new_check_flag_false(offset)?,
            cs: CommandStop::new_check_flag_false(offset)?,
            ca: CommandAbort::new_check_flag_false(offset)?,
            crr: CommandRingRunning::new(offset),
            command_ring_pointer: CommandRingPointer::new(offset),
        })
    }

    pub fn setup_command_ring(&self, allocator: &mut impl MemoryAllocatable) -> PciResult {
        unsafe { allocate_command_ring(self, allocator) }
    }
}
unsafe fn allocate_command_ring(
    crcr: &CommandRingControlRegister,
    allocator: &mut impl MemoryAllocatable,
) -> PciResult {
    const TRB_SIZE: usize = 128;

    let alloc_size = TRB_SIZE * 32;
    let command_ring_addr = allocator
        .alloc(alloc_size)
        .ok_or(PciError::FailedOperateToRegister(FailedAllocate))?;

    register_command_ring(crcr, command_ring_addr as u64)
}

fn register_command_ring(crcr: &CommandRingControlRegister, command_ring_addr: u64) -> PciResult {
    if crcr.cs.read_flag_volatile() || crcr.ca.read_flag_volatile() {
        return Err(PciError::FailedOperateToRegister(
            OperationReason::MustBeCommandRingStopped,
        ));
    }
    crcr.rcs.write_flag_volatile(true);
    crcr.command_ring_pointer
        .set_command_ring_addr(command_ring_addr);
    Ok(())
}

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
