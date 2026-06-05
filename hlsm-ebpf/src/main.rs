#![no_std]
#![no_main]

use aya_ebpf::{macros::lsm, programs::LsmContext};
use aya_log_ebpf::info;

#[lsm(hook = "bprm_check_security")]
pub fn bprm_check_security(ctx: LsmContext) -> i32 {
    match try_bprm_check_security(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_bprm_check_security(ctx: LsmContext) -> Result<i32, i32> {
    info!(&ctx, "lsm hook bprm_check_security called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
