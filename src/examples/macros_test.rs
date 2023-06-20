// #################################################################################################
// Tests
// #################################################################################################

#[cfg(test)]
mod tests {
  use anyhow::Result;

  #[macro_export]
  macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        const fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

  #[macro_export]
  macro_rules! t_bail {
     () => {
         anyhow::bail!(format!("{} {}:{}", file!(), line!(), column!()))
     };
     ($($arg:tt)*) => {{
         anyhow::bail!(format!("{} {}:{} - {}", file!(), line!(), column!(), format_args!($($arg)*)))
     }};
}

  #[macro_export]
  macro_rules! t_anyhow {
     () => {
         anyhow::anyhow!(format!("{} {}:{}", file!(), line!(), column!()))
     };
     ($($arg:tt)*) => {{
         anyhow::anyhow!(format!("{} {}:{} - {}", file!(), line!(), column!(), format_args!($($arg)*)))
     }};
}

  #[macro_export]
  macro_rules! require {
     ($cond:expr) => {
         anyhow::ensure!($cond, format!("{} {}:{}", file!(), line!(), column!()))
     };
     ($cond:expr, $($arg:tt)*) => {{
         anyhow::ensure!($cond, format!("{} {}:{} - {}", file!(), line!(), column!(), format_args!($($arg)*)))
     }};
}


  #[test]
  fn macro_test() {
    assert_eq!(method_1().unwrap(), 1);

    let actual_1 = method_1();
    assert!(actual_1.is_ok());

    method_4().unwrap(); // try to uncomment
    // prints: thread 'examples::error_macros_test::tests::require_enum_test' panicked at 'called `Result::unwrap()` on an `Err` value: src/examples/error_macros_test.rs 91:5 - this must fail

    let actual_2 = method_2();
    assert!(actual_2.is_err());
    assert!(actual_2.err().unwrap().to_string().ends_with(" - 1 != 0"));

    let actual_3 = method_3();
    assert!(actual_3.is_err());
    assert!(actual_3.err().unwrap().to_string().ends_with(" - 42 != 21"));
  }

  fn method_1() -> Result<u32> {
    require!(true, "is true" );
    Ok(1)
  }

  fn method_2() -> Result<u32> {
    require!(1 == 0, "1 != 0" );
    Ok(1)
  }

  fn method_3() -> Result<u32> {
    require!(1 == 0, "{} != {}", 42, 21 );
    Ok(1)
  }

  fn method_4() -> Result<u32> {
    t_bail!("this must fail")
  }
}
