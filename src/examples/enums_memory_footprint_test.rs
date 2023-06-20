#[cfg(test)]
pub mod test {
  use std::mem;

  enum Vehicle {
  Car(CarVehicle),
  Plane(PlaneVehicle)
}

  struct CarVehicle {
    x: i32,
    y: bool
  }

  struct PlaneVehicle {
    x: i32,
  }


  #[test]
  fn enums_memory_footprint_test() {
    println!("memory size of Vehicles {} bytes", mem::size_of::<Vehicle>()); // 8 bytes
    println!("memory size of CarVehicle {} bytes", mem::size_of::<CarVehicle>()); // 8 bytes
    println!("memory size of PlaneVehicle {} bytes", mem::size_of::<PlaneVehicle>()); // 4 bytes

    println!("memory size of Option<Vehicle> {} bytes", mem::size_of::<Option<Vehicle>>()); // 8 bytes
    println!("memory size of Option<PlaneVehicle> {} bytes", mem::size_of::<Option<PlaneVehicle>>()); // 8 bytes
  }
}
