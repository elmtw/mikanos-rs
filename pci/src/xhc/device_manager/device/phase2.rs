use alloc::boxed::Box;
use alloc::vec::Vec;

use xhci::ring::trb::event::TransferEvent;

use crate::class_driver::keyboard::driver::KeyboardDriver;
use crate::class_driver::mouse::driver::MouseDriver;
use crate::error::PciResult;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::control_pipe::request::Request;
use crate::xhc::device_manager::control_pipe::{ControlPipe, ControlPipeTransfer};
use crate::xhc::device_manager::descriptor::descriptor_sequence::DescriptorSequence;
use crate::xhc::device_manager::descriptor::hid::HidDeviceDescriptors;
use crate::xhc::device_manager::descriptor::structs::configuration_descriptor::ConfigurationDescriptor;
use crate::xhc::device_manager::descriptor::structs::interface_descriptor::InterfaceDescriptor;
use crate::xhc::device_manager::descriptor::Descriptor;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase};
use crate::xhc::device_manager::device::phase3::Phase3;
use crate::xhc::registers::traits::doorbell::DoorbellRegistersAccessible;
use crate::xhc::transfer::event::target_event::TargetEvent;

pub struct Phase2 {
    mouse: MouseDriver,
    keyboard: KeyboardDriver,
}


impl Phase2 {
    pub const fn new(mouse: MouseDriver, keyboard: KeyboardDriver) -> Phase2 {
        Self { mouse, keyboard }
    }
}


impl<Doorbell, Memory> Phase<Doorbell, Memory> for Phase2
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellRegistersAccessible + 'static,
{
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Memory, Doorbell>,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> PciResult<(InitStatus, Option<Box<dyn Phase<Doorbell, Memory>>>)> {
        let data_stage = target_event.data_stage()?;

        let conf_desc_buff = data_stage.data_buffer_pointer() as *mut u8;
        let conf_desc_buff_len =
            (data_stage.trb_transfer_length() - transfer_event.trb_transfer_length()) as usize;

        let conf_desc =
            unsafe { *(data_stage.data_buffer_pointer() as *const ConfigurationDescriptor) };
        let descriptors = DescriptorSequence::new(conf_desc_buff, conf_desc_buff_len)
            .collect::<Vec<Descriptor>>();

        let hid_device_descriptors: Vec<HidDeviceDescriptors> = descriptors
            .iter()
            .enumerate()
            .filter_map(filter_interface)
            .filter(|(index, interface)| filter_mouse_or_keyboard((*index, interface)))
            .filter_map(|(index, interface)| map_hid_descriptors(index, interface, &descriptors))
            .collect();

        slot.input_context_mut()
            .set_config(conf_desc.configuration_value);

        set_configuration(
            conf_desc.configuration_value as u16,
            slot.default_control_pipe_mut(),
        )?;

        Ok((
            InitStatus::not(),
            Some(Box::new(Phase3::new(
                self.mouse.clone(),
                self.keyboard.clone(),
                hid_device_descriptors,
            ))),
        ))
    }


    fn interface_nums(&self) -> Option<Vec<u8>> {
        None
    }
}


fn set_configuration<T: DoorbellRegistersAccessible>(
    config_value: u16,
    default_control_pipe: &mut ControlPipe<T>,
) -> PciResult {
    default_control_pipe
        .control_out()
        .no_data(Request::configuration(config_value))
}


fn filter_interface((index, device): (usize, &Descriptor)) -> Option<(usize, InterfaceDescriptor)> {
    device
        .interface()
        .map(|interface| (index, interface.clone()))
}


fn filter_mouse_or_keyboard((_, interface): (usize, &InterfaceDescriptor)) -> bool {
    interface.is_mouse() || interface.is_keyboard()
}


fn map_hid_descriptors(
    index: usize,
    interface: InterfaceDescriptor,
    descriptors: &[Descriptor],
) -> Option<HidDeviceDescriptors> {
    let endpoint = descriptors
        .iter()
        .skip(index + 1 + 1)
        .find_map(|descriptor| {
            if let Descriptor::Endpoint(endpoint) = descriptor {
                Some(endpoint)
            } else {
                None
            }
        })?;
    Some(HidDeviceDescriptors::new(interface, endpoint.clone()))
}
