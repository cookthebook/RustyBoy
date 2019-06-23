extern crate rusty_boy;
use rusty_boy::core::Core;

fn main() {
  let z80 = Core::new();
  z80.print_status();
}
