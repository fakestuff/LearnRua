fn main() {
    let a = [1, 2, 3];
    let c = a.iter()
    .find(|&&ca| ca > 0);
    println!("{:#?}",c);
}
