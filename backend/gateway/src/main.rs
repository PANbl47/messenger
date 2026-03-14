use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = messenger_gateway::app();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("gateway listener should bind");

    axum::serve(listener, app)
        .await
        .expect("gateway server should run");
}
