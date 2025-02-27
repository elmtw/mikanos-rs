use alloc::boxed::Box;
use alloc::vec::Vec;

use xhci::ring::trb::event::TransferEvent;

use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

pub(crate) const DATA_BUFF_SIZE: usize = 256;

/// Configure Commandを送信するか
pub struct InitStatus(bool);


impl InitStatus {
    pub fn new(is_initialized: bool) -> Self {
        Self(is_initialized)
    }
    pub fn not() -> Self {
        Self::new(false)
    }

    pub fn initialized() -> Self {
        Self::new(true)
    }
    pub fn is_initialised(&self) -> bool {
        self.0
    }
}


pub trait Phase<Doorbell, Memory>
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible,
{
    #[allow(clippy::type_complexity)]
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Doorbell, Memory>>>)>;


    fn interface_nums(&self) -> Option<Vec<u8>>;
}
