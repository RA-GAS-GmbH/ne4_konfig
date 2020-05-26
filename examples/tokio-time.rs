use tokio::time::*;


#[tokio::main]
async fn main() {
    println!("Start Task");
    let res = timeout(Duration::from_millis(10), long_run()).await;

    if res.is_err() {
        println!("Operation timed out!");
        return;
    }

    println!("End Task");
}


async fn long_run() {
    delay_for(Duration::from_millis(100)).await;
}
