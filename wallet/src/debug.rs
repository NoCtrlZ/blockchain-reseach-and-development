use std::any::type_name;

pub fn type_of<T>(_: T) {
    println!("{:?}", type_name::<T>());
}
