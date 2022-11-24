#[test]
fn main() {
    let mut v = vec![0, 1];
    let mut_ref_v = &mut v;
    mut_ref_v.push(2);

    let hello = "hello";
    let world = "world";

    println!("{:?}", mut_ref_v);

    println!("string test {} {}", hello, world);

    println!("string test {hello} {world}");

    assert_eq!(v, vec![0, 1, 2])
}
