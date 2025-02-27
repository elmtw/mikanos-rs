use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{InvalidRegisterReason, PciError, PciResult};
use crate::xhc::registers::internal::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;
use crate::xhc::registers::internal::operational_registers::command_ring_control_register::crcr_field::CommandRingControlRegisterField;
use crate::xhc::registers::internal::operational_registers::usb_command_register::run_stop::RunStop;

/// CRR
///
/// RunStopが1でDBReasonがホストコントローラーコマンドに書き込まれている場合、1になります。
///
/// RunStopが0にクリアされるか、CommandStopかCommandAbortに1が書き込まれた後にコマンドリングが停止した場合、
/// 0にクリアされます。
#[derive(VolatileBits)]
#[bits(1)]
#[offset_bit(3)]
pub struct CommandRingRunning(usize, PhantomData<CommandRingControlRegisterOffset>);

impl CommandRingRunning {
    pub fn new_with_check(
        offset: CommandRingControlRegisterOffset,
        run_stop: &RunStop,
    ) -> PciResult<Self> {
        let s = Self::new(offset);
        let xhc_is_run_but_self_is_false = run_stop.read_flag_volatile() && !s.read_flag_volatile();
        if xhc_is_run_but_self_is_false {
            return Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: true },
            ));
        }

        let xhc_is_stop_but_self_is_true = !run_stop.read_flag_volatile() && s.read_flag_volatile();
        if xhc_is_stop_but_self_is_true {
            return Err(PciError::InvalidRegister(
                InvalidRegisterReason::IllegalBitFlag { expect: false },
            ));
        }
        Ok(s)
    }
}
