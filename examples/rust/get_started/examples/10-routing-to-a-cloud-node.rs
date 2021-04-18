use ockam::{Context, Result, Route};
use ockam_transport_tcp::{TcpTransport, TCP};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Create a cloud node by going to https://hub.ockam.network
    let cloud_node = "Paste the address of your cloud node here.";

    // Initialize the TCP Transport.
    let tcp = TcpTransport::create(&ctx).await?;

    // Create a TCP connection to your cloud node.
    tcp.connect(cloud_node).await?;

    // Send a message to the `echo_service` worker on your cloud node.
    ctx.send(
        // route to the echo_service worker on your cloud node
        Route::new()
            .append_t(TCP, cloud_node)
            .append("echo_service"),
        // the message you want echo-ed back
        "Hello Ockam!".to_string(),
    )
    .await?;

    // Wait to receive the echo and print it.
    let msg = ctx.receive::<String>().await?;
    println!("App Received: '{}'", msg); // should print "Hello Ockam!"

    // Stop the node.
    ctx.stop().await
}
