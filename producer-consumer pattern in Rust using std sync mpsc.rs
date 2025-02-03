use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel
    let (tx, rx) = mpsc::channel();

    // Spawn producer threads
    for i in 0..3 {
        let tx_clone = tx.clone(); // Clone the transmitter for each producer thread
        thread::spawn(move || {
            for j in 0..5 {
                let message = format!("Producer {}: Message {}", i, j);
                tx_clone.send(message).unwrap(); // Send the message to the channel
                println!("Producer {} sent Message {}", i, j);
                thread::sleep(Duration::from_millis(100)); // Simulate work
            }
        });
    }

    // Drop the original transmitter to prevent hanging
    drop(tx);

    // Consumer thread
    thread::spawn(move || {
        for received in rx {
            println!("Consumer received: {}", received);
            thread::sleep(Duration::from_millis(150)); // Simulate processing
        }
        println!("All messages processed. Consumer exiting.");
    })
    .join()
    .unwrap();
}



/*


Key Features
Channel-Based Communication:

The mpsc::channel enables safe communication between threads. Producers use the transmitter (tx) to send messages,
 while the consumer receives messages via the receiver (rx).
Cloning Transmitters:

Since mpsc supports multiple producers, we clone the transmitter (tx) for each producer thread. 
This ensures each producer operates independently while sending messages to the same receiver.
Ownership Rules:

In Rust, ownership ensures memory safety even in multi-threaded programs. Here:
Each producer thread moves its tx_clone into the thread's closure, ensuring no concurrent access to shared state.
Messages sent through the channel are also moved, transferring ownership from the producer to the consumer without
 requiring explicit synchronization.
Once the tx is dropped in the main thread, it signals to the receiver that no more messages will be sent, 
preventing the consumer from hanging indefinitely.
Graceful Shutdown:

The drop(tx) ensures no lingering transmitter is left to block the consumer from terminating once all messages are processed.



Explanation (Interview-Style)
**"In this producer-consumer implementation, I used Rust’s std::sync::mpsc 
to facilitate safe communication between threads. Each producer thread clones the transmitter (tx) 
to send messages, and the consumer processes these messages from the receiver (rx).

Ownership in Rust ensures that data passed through the channel is safely moved between threads. 
When a producer sends a message, ownership of that data is transferred to the receiver. 
This prevents any thread from accessing or mutating the same data simultaneously, avoiding data races without needing locks.

I also dropped the main thread's transmitter after spawning the producer threads. 
This signals the consumer that no more messages will arrive once all producers finish, 
enabling a clean shutdown. Rust’s ownership model ensures that even with multiple threads, 
the program remains memory-safe and free of undefined behavior."**


*/