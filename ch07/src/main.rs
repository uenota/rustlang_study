#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn main() {
    let p1 = Parent(1, Child(11), Child(12));

    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }

    println!("(b) p1: {:?}", p1);

    let p3 = Parent(3, Child(31), Child(32));
    println!("(a) p1: {:?}, p3: {:?}", p1, p3);
}
