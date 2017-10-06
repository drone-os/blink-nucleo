#![feature(const_fn)]
#![feature(naked_functions)]
#![no_main]
#![no_std]

extern crate blink_nucleo;
extern crate drone_cortex_m;

use blink_nucleo::{main, VectorTable};
use drone_cortex_m::mcu;

#[no_mangle]
pub static VECTOR_TABLE: VectorTable = VectorTable::new(reset);

#[naked]
unsafe extern "C" fn reset() -> ! {
  #[inline(never)]
  fn handler() {
    main()
  }
  handler();
  loop {
    mcu::wait_for_interrupt();
  }
}
