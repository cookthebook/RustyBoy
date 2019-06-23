pub struct Register {
  val: u8,
}

pub struct Core {
  ix:  Register,
  iy: Register,
  y:  Register,
  a:  Register,
  b:  Register,
  c:  Register,
  d:  Register,
  e:  Register,
  h:  Register,
  l:  Register,
}

impl Register {
  pub fn new() -> Register {
    Register {
      val: 0,
    }
  }

  pub fn set(&mut self, val: u8) {
    self.val = val;
  }

  pub fn get(&self) -> u8 {
    self.val
  }
}

impl Core {
  pub fn new() -> Core {
    Core {
      ix: Register::new(),
      iy: Register::new(),
      y: Register::new(),
      a: Register::new(),
      b: Register::new(),
      c: Register::new(),
      d: Register::new(),
      e: Register::new(),
      h: Register::new(),
      l: Register::new(),
    }
  }

  pub fn print_status(&self) {
    println!("CORE STATUS");
    println!("-----------");
    println!("Registers");
    println!("ix:\t{}", self.ix.get());
    println!("iy:\t{}", self.iy.get());
    println!("y:\t{}", self.y.get());
    println!("a:\t{}", self.a.get());
    println!("b:\t{}", self.b.get());
    println!("c:\t{}", self.c.get());
    println!("d:\t{}", self.d.get());
    println!("e:\t{}", self.e.get());
    println!("h:\t{}", self.h.get());
    println!("l:\t{}", self.l.get());
  }

  pub fn ld0(&self, r: &mut Register, rp: &Register) {
    r.set(rp.get());
  }

  pub fn ld1(&self, r: &mut Register, n: u8) {
    r.set(n);
  }

  pub fn inc0(&self, r: &mut Register) {
    r.set(r.get() + 1);
  }

  pub fn add0(&mut self, r: &Register) {
    self.a.set(self.a.get() + r.get());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ld0() {
    let core = Core::new();
    let mut r = Register::new();
    let mut rp = Register::new();
    r.set(0);
    rp.set(1);
    core.ld0(&mut r, &rp);
    assert_eq!(r.get(), 1);
    assert_eq!(rp.get(), 1);
  }

  #[test]
  fn test_1d1() {
    let core = Core::new();
    let mut r = Register::new();
    r.set(0);
    core.ld1(&mut r, 1);
    assert_eq!(r.get(), 1);
  }

  #[test]
  fn test_inc0() {
    let core = Core::new();
    let mut r = Register::new();
    r.set(0);
    core.inc0(&mut r);
    assert_eq!(r.get(), 1);
  }

  #[test]
  fn test_add0() {
    let mut core = Core::new();
    let mut r = Register::new();
    r.set(1);
    core.add0(&r);
    assert_eq!(core.a.get(), 1);
  }
}
