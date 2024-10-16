pub fn set_key_irq(enable: bool) {
    unsafe {
        if enable {
            *(nitro_sys::REG_IE_ADDR as *mut u32) |= 1 << 12;
        } else {
            *(nitro_sys::REG_IE_ADDR as *mut u32) &= !(1 << 12);
        }
        *(nitro_sys::REG_IF_ADDR as *mut u32) |= 1 << 12;
    }
}

// 由于 DeSmuMe 模拟器对 ARM 指令集中的 SWI 指令似乎存在错误 JIT 编译导致执行出错
// 故固定使用 Thumb 指令集实现以免卡死
#[cfg_attr(target_arch = "arm", instruction_set(arm::t32))]
pub fn wait_vblank() {
    // #[cfg(target = "armv5te-none-eabi")]
    // unsafe {
    //     core::arch::asm!(
    //         "push {{r0-r2}}",
    //         "mov r0, 0",
    //         "mov r1, 0",
    //         "sub r1, 1",
    //         "mov r2, 0",
    //         "swi 0x050000",
    //         "pop {{r0-r2}}",
    //         options(nomem, nostack),
    //     )
    // }
    // #[cfg(target = "thumbv5te-none-eabi")]
    unsafe { core::arch::asm!("swi 0x05") }
}

// 由于 DeSmuMe 模拟器对 ARM 指令集中的 SWI 指令似乎存在错误 JIT 编译导致执行出错
// 故固定使用 Thumb 指令集实现以免卡死
#[cfg_attr(target_arch = "arm", instruction_set(arm::t32))]
pub fn wait_any_intr() {
    // #[cfg(target = "armv5te-none-eabi")]
    // unsafe {
    //     core::arch::asm!(
    //         "push {{r0-r2}}",
    //         "mov r0, 0",
    //         "mov r1, 0",
    //         "sub r1, 1",
    //         "mov r2, 0",
    //         "swi 0x040000",
    //         "pop {{r0-r2}}",
    //         options(nomem, nostack),
    //     )
    // }
    // #[cfg(target = "thumbv5te-none-eabi")]
    unsafe {
        core::arch::asm!(
            "movs r0, #0",
            "movs r1, #0",
            "subs r1, #1",
            "movs r2, #0",
            "swi 0x04",
            options(nomem, nostack),
        )
    }
}
