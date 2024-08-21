use tokio::runtime;

async fn async_main() {
    println!("Hello, world!");
}

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async_main());
}
