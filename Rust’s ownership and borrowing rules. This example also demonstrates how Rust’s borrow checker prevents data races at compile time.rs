use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared data structure: a vector inside an Arc<Mutex>
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));

    // Create multiple threads to manipulate the shared data
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data); // Clone the Arc for each thread

        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap(); // Lock the Mutex to get mutable access
            data.push(i * 10); // Modify the shared data
            println!("Thread {} added {}", i, i * 10);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the shared data after all threads complete
    println!("Final shared data: {:?}", *shared_data.lock().unwrap());
}


/*





Key Points of the Implementation
Shared Data Structure:

The Arc<Mutex<Vec<i32>>> is used to share a vector (Vec<i32>) safely across threads.
Arc (Atomic Reference Counting): Enables multiple threads to share ownership of the vector.
Mutex: Ensures mutual exclusion, allowing only one thread at a time to access the vector.
Borrowing Rules:

Each thread receives a clone of the Arc, ensuring ownership semantics are maintained.
The Mutex ensures mutable access to the vector, but it enforces that only one thread can modify the How the Borrow Checker Prevents Data Races


How the Borrow Checker Prevents Data Races : 

**"In this example, Rust’s borrow checker works in tandem with Arc and Mutex to prevent data races:

Ownership and borrowing rules ensure that each thread has a safe, immutable reference to the shared Arc.
When accessing the Mutex-protected data, the borrow checker enforces exclusive access through the lock, preventing simultaneous modifications by multiple threads.
Rust’s type system ensures that the Mutex lock is always released properly, even in case of panics, because it implements RAII (Resource Acquisition Is Initialization).
These features allow us to safely manipulate shared data across threads without risking undefined behavior or runtime errors caused by data races."**

Output Example
When the program runs, you’ll see output like:



Thread 0 added 0
Thread 1 added 10
Thread 2 added 20
Final shared data: [1, 2, 3, 4, 5, 0, 10, 20]data at a time by requiring a lock before accessing the data.
Borrow Checker:

The borrow checker enforces Rust’s no data races guarantee at compile time by:
Ensuring only one mutable reference exists at a time.
Preventing simultaneous reads and writes to the shared data.
Concurrency:

While threads run concurrently, the Mutex ensures synchronized access to the vector. If a thread attempts to acquire the lock while another thread holds it, the thread will block until the lock becomes available.










*/