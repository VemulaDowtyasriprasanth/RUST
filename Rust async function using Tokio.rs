use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt};
use std::io;

async fn read_from_socket(socket: TcpStream, timeout_duration: Duration) -> io::Result<String> {
    // Allocate a buffer for the incoming data
    let mut buffer = vec![0; 1024];

    // Wrap the read operation with a timeout
    let result = timeout(timeout_duration, socket.read(&mut buffer)).await;

    match result {
        Ok(Ok(bytes_read)) if bytes_read > 0 => {
            Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
        }
        Ok(Ok(_)) => Ok(String::from("Connection closed by peer")),
        Ok(Err(e)) => Err(io::Error::new(io::ErrorKind::Other, format!("Read error: {}", e))),
        Err(_) => Err(io::Error::new(io::ErrorKind::TimedOut, "Operation timed out")),
    }
}

async fn process_multiple_sockets(sockets: Vec<TcpStream>, timeout_duration: Duration) {
    let mut tasks = Vec::new();

    // Spawn async tasks for each socket
    for (i, socket) in sockets.into_iter().enumerate() {
        let task = tokio::spawn(async move {
            match read_from_socket(socket, timeout_duration).await {
                Ok(data) => println!("Socket {}: Data received: {}", i, data),
                Err(e) => println!("Socket {}: Error: {}", i, e),
            }
        });
        tasks.push(task);
    }

    // Await all tasks
    for task in tasks {
        if let Err(e) = task.await {
            println!("A task panicked: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Example setup: Connect to some example sockets
    let sockets = vec![
        TcpStream::connect("example.com:80").await?,
        TcpStream::connect("example.com:8080").await?,
    ];

    // Process sockets concurrently with a timeout of 5 seconds per socket
    process_multiple_sockets(sockets, Duration::from_secs(5)).await;

    Ok(())
}






/*


Key Features in the Code
Concurrency:

Each socket is processed concurrently using tokio::spawn, ensuring non-blocking behavior and optimal resource usage.
Timeout Handling:

The timeout function from tokio::time ensures that read operations don’t block indefinitely. If a timeout occurs, it is explicitly handled with an appropriate error message.
Cancellation Handling:

Tasks are cancellable via tokio::spawn. If the main task or runtime is canceled, all spawned tasks will stop gracefully.
Graceful Error Management:

All potential errors (e.g., timeouts, I/O issues, or socket closures) are captured and logged with detailed messages.
Explanation (Interview-Style)
**"In this implementation, I used Tokio to manage asynchronous socket reads. Each socket is handled concurrently via tokio::spawn, which ensures efficient resource utilization by leveraging Tokio’s lightweight task scheduler.

I introduced tokio::time::timeout to handle cases where a socket read operation hangs for too long. If the operation exceeds the specified timeout, it’s cleanly aborted with an appropriate error message. For robust error handling, the function captures and distinguishes between different error types—such as I/O errors, timeouts, or connection closures.

Cancellation is inherently supported since the tasks spawned with tokio::spawn will terminate if the parent runtime or task is canceled. This design ensures both performance and resilience in scenarios where multiple sockets must be managed concurrently."**






*/
