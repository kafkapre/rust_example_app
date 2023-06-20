#[cfg(test)]
pub mod test {
  #[test]
  fn variable_test() {
    let x = 3;
    let mut y = 1;

    println!("x={x}");
    println!("y={y}");

    y = 5;

    println!("y={y}");
  }

}