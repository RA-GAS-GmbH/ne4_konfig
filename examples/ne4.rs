use ne4_konfig::sensors::ne4::*;

#[tokio::main]
async fn main() {
    let mut ne4 = NE4::new();
    println!("{:?}", ne4);
    ne4.update().await;
    println!("{:?}", ne4);
}
