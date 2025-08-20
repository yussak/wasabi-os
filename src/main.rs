// stdライブラリ（クレート）を使わない宣言
#![no_std]
// main関数を使わない宣言
#![no_main]

// 名前修飾（マングリング）の無効化
#[no_mangle]
fn efi_main(_image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let efi_graphics_output_protocol = locate_graphics_protocol(efi_system_table).unwrap();
    let vram_addr = efi_graphics_output_protocol.mode.frame_buffer_base;
    let vram_byte_size = efi_graphics_output_protocol.mode.frame_buffer_size;
    let vram = unsafe {
        slice::from_raw_parts_mut(vram_addr as *mut u32, vram_byte_size / size_of::<u32()>)
    };
    for e in vram {
        *e = 0xffffff;
    }

    // println!("Hello, world!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
