use super::uart;

#[macro_export]
macro_rules! println {
  ($($arg:tt)*) => {
      {
          use core::fmt::Write;
          writeln!($crate::Printer, $($arg)*).ok();
      }
  };
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {
      {
          use core::fmt::Write;
          write!($crate::Printer, $($arg)*).ok();
      }
  };
}

pub struct Printer;

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart::write_str(s);
        core::fmt::Result::Ok(())
    }
}
