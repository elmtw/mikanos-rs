use volatile_bits::{VolatileAddress, VolatileBitsReadable, VolatileBitsWritable};

use crate::apic::device_config::LocalApicTimerDivide;
use crate::apic::lvt_timer::timer_mode::TimerMode::Periodic;
use crate::apic::lvt_timer::LvtTimerAddr;
use crate::apic::LocalApicRegisters;
use crate::interrupt::interrupt_vector::InterruptVector;
use crate::serial_println;
use crate::timer::apic::ApicTimer;

#[derive(Default)]
pub struct LocalApicTimer {
    local_apic_registers: LocalApicRegisters,
}


impl LocalApicTimer {
    pub fn new() -> Self {
        let local_apic_registers = LocalApicRegisters::default();

        Self {
            local_apic_registers,
        }
    }
}


impl ApicTimer for LocalApicTimer {
    fn start(&mut self, divide: LocalApicTimerDivide) {
        self.local_apic_registers
            .divide_config()
            .update_divide(divide);


        let lvt_timer = self
            .local_apic_registers
            .lvt_timer();

        lvt_timer
            .interrupt_id_num()
            .write_volatile(InterruptVector::ApicTimer as u32)
            .unwrap();

        lvt_timer
            .mask()
            .write_volatile(0)
            .unwrap();

        lvt_timer
            .timer_mode()
            .update_timer_mode(Periodic);

        self.local_apic_registers
            .initial_count()
            .write_volatile(u32::MAX)
            .unwrap();


        serial_println!("Timer 0x{:b}", unsafe {
            core::ptr::read_volatile(LvtTimerAddr::default().address() as *const u32)
        });
    }


    fn elapsed(&self) -> u32 {
        let current = self
            .local_apic_registers
            .current_count()
            .read_volatile();

        u32::MAX - current
    }


    fn stop(&mut self) {
        self.local_apic_registers
            .initial_count()
            .write_volatile(0)
            .unwrap();
    }
}
