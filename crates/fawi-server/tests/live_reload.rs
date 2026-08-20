//! End-to-end test for the websocket live-reload path.

use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn websocket_reports_bundle_change() {
    let root = std::env::temp_dir().join(format!("okf-live-reload-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("a.md"), "---\ntype: T\ntitle: A\n---\nbody\n").unwrap();

    let bundle = fawi_storage::FsBundle::open(&root).await.unwrap();
    fawi_server::api::init_bundle(bundle);

    let app = fawi_server::api::router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Let the server and watcher start.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let url = format!("ws://{addr}/api/ws");
    let (ws, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
    let (mut sink, mut stream) = ws.split();

    sink.send(Message::Text(r#"{"type":"watch","path":""}"#.into()))
        .await
        .unwrap();

    // Give the server a moment to register the watch and the watcher to arm.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    std::fs::write(root.join("a.md"), "---\ntype: T\ntitle: A2\n---\nbody2\n").unwrap();

    let msg = tokio::time::timeout(std::time::Duration::from_secs(5), stream.next())
        .await
        .expect("timed out waiting for websocket change message")
        .expect("websocket closed")
        .expect("websocket error");

    let text = msg.into_text().expect("expected text message");
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(value["type"], "change");
    assert_eq!(value["path"], "");
    assert_eq!(value["paths"], serde_json::json!(["a"]));

    server.abort();
    let _ = std::fs::remove_dir_all(&root);
}
