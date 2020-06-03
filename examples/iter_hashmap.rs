fn main() {
    let values = vec!["a", "b", "c"];

    for (i, p) in (0u32..).zip(values.into_iter()) {
        println!("i: {:?}/ p: {:?}", i, p);
    }
}
