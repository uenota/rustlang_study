//use std::ops::Drop;

#[derive(Copy, Clone, Debug)]
struct Parent(usize, Child, Child);

#[derive(Copy, Clone, Debug)]
struct Child(usize);

// impl Drop for Parent {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self);
//     }
// }

// impl Drop for Child {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self);
//     }
// }

fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    println!("p1: {:?}", p1); // this causes error

    let p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);
}
