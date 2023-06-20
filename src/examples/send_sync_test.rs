#[cfg(test)]
pub mod test {
  use std::cell::RefCell;
  use std::rc::Rc;
  use std::sync::Arc;

  struct StructA {
    x: i32,
  }

  struct StructB {
    x: Arc<i32>,
  }

  struct StructC {
    x: RefCell<i32>,
  }

  struct StructD {
    x: Rc<i32>,
  }

  struct StructE {
    x: Box<i32>,
  }


  #[test]
  fn send_sync_test() {
    is_send::<StructA>();
    is_send_and_sync::<StructA>();

    is_send::<StructB>();
    is_send_and_sync::<StructB>();

    // is_send::<StructC>();
    // is_send_and_sync::<StructC>();
    //
    // is_send::<StructD>();
    // is_send_and_sync::<StructD>();
    //
    // is_send::<StructE>();
    // is_send_and_sync::<StructE>();
  }

  pub fn is_send<T: Send>() {}

  pub fn is_send_and_sync<T: Send + Sync>() {}
}
