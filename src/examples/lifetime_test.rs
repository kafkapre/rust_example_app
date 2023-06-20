#[cfg(test)]
pub mod test {
  struct Car<'a> {
    name: &'a str,
    age: u32,
  }


  #[test]
  fn lifetime_test() {
    let name = "octavia";
    let motor = "TDI";

    bar(name, motor);
    foo(name, motor);

    // let car = Car{ "does not work", age: 0 };
    let car1 = Car { name, age: 0 };

    let car2 = func_returning_car();

    let car3 = func_building_car(name);

    // func_uncompilable_car();

    let car4 = func_const_car();
  }


  fn bar<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() > 5 { s } else { t }
  }

  fn foo<'a, 'b>(s: &'a str, _t: &'b str) -> &'a str {
    s
  }

  fn func_returning_car<'a>() -> Car<'a> {
    let name = "fabue";
    let car = Car { name, age: 0 };
    car
  }

  fn func_building_car<'a>(name: &'a str) -> Car<'a> {
    let car = Car { name, age: 0 };
    car
  }

  // fn func_uncompilable_car<'a>(name: &'a str) -> &Car<'a> {
  //   let car = Car { name, age: 0 };
  //   &car
  // }

  fn func_const_car() -> &'static Car<'static> {
    &Car { name: "fabue", age: 0 }
  }

}
