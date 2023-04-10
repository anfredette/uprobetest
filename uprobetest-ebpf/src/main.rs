#![no_std]
#![no_main]

use aya_bpf::{
    macros::uprobe,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

#[uprobe(name="uprobetest")]
pub fn uprobetest(ctx: ProbeContext) -> u32 {
    match try_uprobetest(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_uprobetest(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function main called by hello");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
