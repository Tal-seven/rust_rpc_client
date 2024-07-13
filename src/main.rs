pub mod proto {
    tonic::include_proto!("rust_rmq.sensors");
}

use std::error::Error;
use tokio_stream::StreamExt;
use tonic::transport::Channel;

use proto::{scan_service_client::ScanServiceClient, Empty};

async fn streaming_scan(client: &mut ScanServiceClient<Channel>) {
    let mut stream = client.get_scan_stream(Empty {}).await.unwrap().into_inner();

    while let Some(scan) = stream.next().await {
        println!("Received scan value {:?}", scan);
        if let Ok(value) = scan {
            println!(
                "\treceived: AngleMax: {} AngleMin: {}, ScanTime: {}",
                value.angle_max, value.angle_min, value.scan_time
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ScanServiceClient::connect("http://localhost:8080")
        .await
        .unwrap();

    println!("Streaming Scan msgs");
    let t1 = tokio::spawn(async move { streaming_scan(&mut client).await });
    let _ = t1.await;
    Ok(())
}
