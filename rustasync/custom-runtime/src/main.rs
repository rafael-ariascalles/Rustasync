use tokio::runtime;

async fn async_main() {
    println!("Hello, world!");
}

fn main() {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(8)
        .build()
        .unwrap();

    rt.block_on(async_main());
}
