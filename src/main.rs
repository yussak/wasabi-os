// stdライブラリ（クレート）を使わない宣言
#![no_std]
// main関数を使わない宣言
#![no_main]

// 名前修飾（マングリング）の無効化
#[no_mangle]
fn efi_main() {
    // println!("Hello, world!");
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
