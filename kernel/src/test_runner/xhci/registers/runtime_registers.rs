use crate::test_runner::xhci::mmio_base_addr;
use pci::xhc::registers::internal::capability_registers::capability_length::CapabilityLength;
use pci::xhc::registers::internal::capability_registers::runtime_register_space_offset::RuntimeRegisterSpaceOffset;
use pci::xhc::registers::internal::runtime_registers::interrupter_register_set::InterrupterRegisterSetOffset;
use pci::xhc::registers::internal::runtime_registers::RuntimeRegistersOffset;

mod interrupter_register_set;

pub(crate) fn runtime_registers_offset() -> RuntimeRegistersOffset {
    let rts_off = RuntimeRegisterSpaceOffset::new_with_check_size(
        mmio_base_addr(),
        &CapabilityLength::new_check_length(mmio_base_addr()).unwrap(),
    )
    .unwrap();

    RuntimeRegistersOffset::new(mmio_base_addr(), &rts_off)
}

pub(crate) fn interrupter_register_set_offset(index: usize) -> InterrupterRegisterSetOffset {
    InterrupterRegisterSetOffset::new(runtime_registers_offset(), index)
}
