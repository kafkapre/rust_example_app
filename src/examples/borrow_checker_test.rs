#[cfg(test)]
pub mod test {
  #[derive(Clone, Debug)]
  struct StructA {
    x: i32,
  }

  #[test]
  fn borrow_checker_1_test() {
    // let x = 5;
    // let y = &x;
    // println!("{}", y); // output: 5
    //
    // let z;
    // {
    //   let w = 10;
    //   z = &w;
    // }
    // println!("{}", z); // error: borrowed value does not live long enough
  }


  #[test]
  fn borrow_checker_2_test() {
    // let x = get_from_mut_var();
    // x += 1;
    //
    // let y = StructA { x: 1 };
    // take_ownership_method(y);
    // println!("y={y}");
    //
    // take_ownership_method(y.clone());
    // println!("y={:?}", y);
    //
    // borrow_method(&y);
    // println!("y={:?}", y);
  }

  fn get_from_mut_var() -> i32 {
    let mut res = 4;
    res += 1;
    res
  }

  fn take_ownership_method(x: StructA) {
    println!("{:?}", x)
  }

  fn borrow_method(x: &StructA) {
    println!("{:?}", x)
  }

  fn variable_shadowing_test() {
    let x = 42.0;
    println!("{x}");

    let x = true;
    println!("{x}");
  }
}