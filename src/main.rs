use axum::{routing::get, Json, Router};
use serde_json::json;
use sysinfo::System;

async fn memfetch() -> Json<serde_json::Value> {
    let mut sys = System::new_all();
    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Fetch memory usage
    let total_memory = sys.total_memory(); // in bytes
    let used_memory = sys.used_memory(); // in bytes
    let free_memory = sys.free_memory();

    // // Byte -> KB conversion
    let total_memory = total_memory/1024;
    let used_memory = used_memory/1024;
    let free_memory = free_memory/1024;

    Json(json!({
        "total_memory_kb": total_memory,
        "used_memory_kb": used_memory,
        "free_memory_kb": free_memory
    }))
}

#[tokio::main]
async fn main() {
   let app: Router = Router::new().route("/memory", get(memfetch));
    // Specify the address and port
    
    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    println!("Server running on http://{}", local_addr);
    axum::serve(listener, app).await.unwrap();
}
