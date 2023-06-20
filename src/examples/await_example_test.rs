#[cfg(test)]
pub mod test {

  #[tokio::test]
 async fn await_test() {
    f().await;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("FINISHED")
  }

  async fn g() {
    println!("hello");
  }

  async fn f() {
    g().await;
  }

}