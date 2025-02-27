use kernel_lib::task::message::TaskMessage;
use kernel_lib::task::TASK_MANAGER;
use kernel_lib::timer::TIME_HANDLE_MANAGER;
use pci::class_driver::mouse::driver::MouseDriver;
use pci::class_driver::mouse::subscribable::MouseSubscribable;
use pci::xhc::allocator::mikanos_pci_memory_allocator::MikanOSPciMemoryAllocator;
use pci::xhc::registers::external::{External, IdentityMapper};
use pci::xhc::registers::memory_mapped_addr::MemoryMappedAddr;
use pci::xhc::XhcController;

use crate::apic::TIMER_200_MILLI_INTERVAL;
use crate::task::task_message_iter::TaskMessageIter;
use crate::usb::keyboard::build_keyboard_driver;

pub fn start_xhci_host_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<()> {
    unsafe {
        crate::task::init();
        TIME_HANDLE_MANAGER.entry(TIMER_200_MILLI_INTERVAL, || {
            TASK_MANAGER.switch().unwrap();
        });
    }

    let mut xhc_controller = start_xhc_controller(mmio_base_addr, mouse_subscriber)?;

    let messages = TaskMessageIter::new(0);
    messages.for_each(|message| match message {
        TaskMessage::Xhci => {
            xhc_controller.process_all_events();
        }

        TaskMessage::Dispatch(handler) => {
            handler();
        }
    });

    Ok(())
}


fn start_xhc_controller(
    mmio_base_addr: MemoryMappedAddr,
    mouse_subscriber: impl MouseSubscribable + 'static,
) -> anyhow::Result<XhcController<External<IdentityMapper>, MikanOSPciMemoryAllocator>> {
    let registers = External::new(mmio_base_addr, IdentityMapper);
    let allocator = MikanOSPciMemoryAllocator::new();

    let mut xhc_controller = XhcController::new(
        registers,
        allocator,
        MouseDriver::new(mouse_subscriber),
        build_keyboard_driver(),
    )
    .map_err(|_| anyhow::anyhow!("Failed initialize xhc controller"))?;

    xhc_controller
        .reset_port()
        .map_err(|e| e.inner())?;

    Ok(xhc_controller)
}
