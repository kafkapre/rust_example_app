#[cfg(test)]
pub mod test {

  #[test]
  fn string_test() {
    let s1 = "hello 1";
    let s2 = "hello 2".to_string();
    let s3 = String::from("hello 3");

    pass_string(s1);
    pass_string(s2);

    println!("{}", static_string());
    // println!("{}", ref_to_string());
  }

  fn static_string() -> &'static str {
    "hello world"
  }

  // fn ref_to_string() -> &str {
  //   "hello world"
  // }

  fn pass_string(str: impl AsRef<str>) {
    println!("{}", str.as_ref())
  }
}
