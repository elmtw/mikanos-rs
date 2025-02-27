use core::marker::PhantomData;

use macros::VolatileBits;

use crate::error::{PciError, PciResult};
use crate::error::OperationReason::MustBeCommandRingStopped;
use crate::xhc::registers::internal::operational_registers::command_ring_control_register::command_ring_running::CommandRingRunning;
use crate::xhc::registers::internal::operational_registers::command_ring_control_register::CommandRingControlRegisterOffset;

/// CA
///
/// このレジスタに1を書き込むと現在のコマンドの実行を直ちに終了し、コマンドリングの動作を停止します。
/// その後、CommandRingStoppedを表すCompletionCodeを生成します。
///
/// Note: 読み込むときは常に0になります。
///
/// Note: CommandRingRunning(CRR)が1の場合、書き込みは無視されます。
#[derive(VolatileBits)]
#[bits(1)]
#[offset_bit(2)]
pub struct CommandAbort(usize, PhantomData<CommandRingControlRegisterOffset>);

impl CommandAbort {
    pub fn abort_command(crr: &CommandRingRunning) -> PciResult {
        if crr.read_flag_volatile() {
            return Err(PciError::FailedOperateToRegister(MustBeCommandRingStopped));
        }

        crr.write_flag_volatile(true);
        Ok(())
    }
}
