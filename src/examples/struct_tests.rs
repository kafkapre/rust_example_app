#[cfg(test)]
pub mod test {
  use std::fmt::{Display, Formatter};
  use std::mem;

  #[derive(Copy, Clone, PartialEq, Hash, Debug)]
  struct StructA {
    x: i32,
    pub y: i32,
  }

  impl StructA {
    fn new(x: i32, y: i32) -> Self {
      StructA { x, y }
    }

    fn sum(&self) -> i32 {
      self.x + self.y
    }
  }

  impl Display for StructA {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
      write!(fmt, "StructA(x:{}, y: {})", self.x, self.y)
    }
  }

  #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
  struct StructB(i32);

  #[derive(Clone, Debug)]
  struct StructC {
    exists: bool,
    x: f32,
    y: i8,
    _private: (),
  }

  #[derive(Clone, PartialEq, Debug)]
  struct StructD {
    x: f32,
    collection: Vec<i32>,
  }

  #[test]
  fn struct_test() {
    let a = StructA::new(1, 2);
    let b = StructB(3);
    let c = StructC { exists: true, x: 1.0, y: 2, _private: () };

    println!("a={a}");
    println!("b={:?}", b);
    println!("c={:?}", c);
    println!("sum a={}", a.sum());
    println!();
    println!("memory size of StructA {} bytes", mem::size_of::<StructA>());
    println!("memory size of StructB {} bytes", mem::size_of::<StructB>());
    println!("memory size of StructC {} bytes", mem::size_of::<StructC>());
  }

  #[test]
  fn struct_equal_test() {
    let a = StructA::new(1, 2);
    let b = StructA::new(1, 2);
    let c = StructA::new(11, 22);

    let z = StructD { x: 0.0, collection: vec![1, 2] };
    let y = StructD { x: 0.0, collection: vec![1, 2] };
    let x = StructD { x: 0.0, collection: vec![1, 2, 3] };

    println!("a == b : {}", a == b);
    println!("a == c : {}", a == c);

    println!("z == y : {}", z == y);
    println!("z == x : {}", z == x);
  }

  #[test]
  fn struct_ordering_test() {
    let mut vec = Vec::with_capacity(3);
    vec.push( StructB(3));
    vec.push( StructB(42));
    vec.push( StructB(1));

    println!("vec unsorted={:?}", vec);

    vec.sort();

    println!("vec sorted={:?}", vec);
  }

  fn struct_copy_clone_test() {
    let a = StructA::new(1, 2);
    let b = StructB(1);


    // pass_struct_a(a);
    // pass_struct_a(a.clone());
    //
    // pass_struct_b(b);
    // pass_struct_b(b.clone());
    // pass_struct_b_by_ref(&b);
    //

    let a_updated_v1 = StructA{
      x: 42,
      ..a // compiler optimize it, no copy/clone is done if it is necessary
    };

    let a_updated_v2 = StructA{
      y: 24,
      ..a.clone() // compiler optimize it, no copy/clone is done if it is necessary
    };

  }

  fn pass_struct_a(x: StructA) {
    println!("{:?}", x)
  }

  fn pass_struct_b(x: StructB) {
    println!("{:?}", x)
  }

  fn pass_struct_b_by_ref(x: &StructB) {
    println!("{:?}", x)
  }

}
