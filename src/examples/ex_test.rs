#[cfg(test)]
pub mod test {
  use std::sync::{Arc, Mutex};

  #[test]
  fn compare_pointers_of_static_constant_created_by_method_test() {
    let s1 = static_string();
    let s2 = static_string();

    let s4 = "hello world";
    let s5 = "hello world 2";


    println!("s1: {:p}", s1);
    println!("s2: {:p}", s2);

    println!("s4: {:p}", s4);
    println!("s5: {:p}", s5);
  }

  #[tokio::test]
  async fn compare_pointers_of_static_constant_created_by_method_async_test() {
    let s1_p = Arc::new(Mutex::new("s1: null".to_string()));
    let s2_p = Arc::new(Mutex::new("s2: null".to_string()));
    let s3_p = Arc::new(Mutex::new("s3: null".to_string()));

    let s1_p_cl = s1_p.clone();
    let s2_p_cl = s2_p.clone();
    let s3_p_cl = s3_p.clone();

    tokio::spawn(async move {
      let mut mux = s1_p.lock().unwrap(); // using is Mutex is just for example how to use it.
      *mux = format!("s1: {:p}", static_string());
    }).await.unwrap();

    tokio::spawn(async move {
      let mut mux = s2_p.lock().unwrap();
      *mux = format!("s2: {:p}", static_string());
    }).await.unwrap();

    tokio::spawn(async move {
      let mut mux = s3_p.lock().unwrap();
      *mux = format!("s3: {:p}", "hello world");
    }).await.unwrap();

    println!("{}", s1_p_cl.lock().unwrap());
    println!("{}", s2_p_cl.lock().unwrap());
    println!("{}", s3_p_cl.lock().unwrap());
  }

  fn static_string() -> &'static str {
    "hello world"
  }
}
