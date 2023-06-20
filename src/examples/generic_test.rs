#[cfg(test)]
pub mod test {
  use anyhow::Result;

  trait Parent {
    fn some_awsome_function(&self) -> Result<i32>;
  }

  #[derive(Clone, Debug)]
  struct Child {
    x: i32,
  }

  impl Child {
    fn own_function() {
      println!("own function")
    }
  }

  impl Parent for Child {
    fn some_awsome_function(&self) -> Result<i32> {
      println!("some owsome function");
      Ok(self.x * 2)
    }
  }

  #[test]
  fn generic_test() {
    let child = Child { x: 3 };
    child.some_awsome_function().unwrap();

    // generic_function(child).unwrap();
    // generic_imp_function(child).unwrap();
    // dynamic_dispatch_function(&child).unwrap();
  }

  fn generic_function<T: Parent>(x: T) -> Result<i32> {
    x.some_awsome_function()
  }

  fn generic_imp_function(x: impl Parent) -> Result<i32> {
    x.some_awsome_function()
  }

  fn dynamic_dispatch_function(x: &dyn Parent) -> Result<i32> {
    x.some_awsome_function()
  }
}
