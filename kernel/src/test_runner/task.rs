use kernel_lib::context::arch::x86_64::Context;
use kernel_lib::serial_println;

#[test_case]
#[allow(clippy::fn_to_numeric_cast)]
fn it_switch_task_context() {
    unsafe {
        static TASK_A_CTX: Context = Context::uninit();
        static mut TASK_B_CTX: Context = Context::uninit();

        let task_b_stack: [u64; 1024] = [0; 1024];
        let task_b_stack_end = task_b_stack
            .as_ptr_range()
            .end as u64;

        unsafe extern "sysv64" fn task(id: u32, data: u32) {
            serial_println!("1. Start Task B id = {} data = {}", id, data);
            TASK_B_CTX.switch_to(&TASK_A_CTX);
            panic!("Do not reach this code.")
        }

        TASK_B_CTX.init_context(task as u64, 0, 1, (task_b_stack_end & !0xF) - 8);

        TASK_A_CTX.switch_to(&TASK_B_CTX);
        serial_println!("2. Back to Task A");
    }
}
