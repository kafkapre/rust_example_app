#[cfg(test)]
pub mod test {

  #[test]
  fn reference_test() {
    let x = 1;
    let reference = &x;
    let mut y = 13;

    println!("x={x}");
    println!("x={reference}");
    my_function(&x);
    my_function(reference);

    println!("x={x}");
    println!("y={y}");

    my_mut_function(&mut y);

    println!("y={y}");
  }

  fn my_function(x: &i32) {
    println!("x={x}");
  }

  fn my_mut_function(x: &mut i32) {
    *x = 42
  }

}
