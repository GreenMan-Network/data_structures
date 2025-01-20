# Data Structures Library

This library contains a collection of data structures implemented in Rust. It is designed to be freely used and serves as a laboratory for understanding the usage of references in Rust. The library exclusively uses safe references to ensure memory safety and prevent common issues such as dangling pointers and memory leaks.

## Purpose

The primary purpose of this library is to provide a set of data structures that can be used in various applications.

Additionally, it serves as a learning tool for understanding how to work with references in Rust, including the use of `Rc`, `RefCell`, and `Weak` references.

## Data Structures

### 1. Circular Queue

A circular queue is a linear data structure that follows the FIFO (First In First Out) principle but connects the end of the queue back to the front, forming a circle. This implementation uses a doubly linked list of blocks, where each block can point to its neighboring blocks.

**Use Cases:**
- Task scheduling
- Buffer management
- Round-robin scheduling

### 2. Block

A block is a fundamental building block for linked data structures. Each block contains data and pointers to the next and previous blocks. This implementation uses `Rc<RefCell<Block<T>>>` for safe reference counting and interior mutability.

**Use Cases:**
- Linked lists
- Doubly linked lists
- Circular linked lists

## Safe References

This library exclusively uses safe references to manage memory. The following types are used to ensure safety:

- `Rc`: A reference-counted smart pointer that enables multiple ownership of data.
- `RefCell`: A type that provides interior mutability, allowing data to be mutated even when the `RefCell` itself is immutable.
- `Weak`: A non-owning reference that does not increment the reference count, used to prevent reference cycles.

## Example Usage

Here is an example of how to use the circular queue:

```rust
use data_structures::circular_queue::{CircularQueue, Side};

fn main() {
    let mut queue: CircularQueue<i32> = CircularQueue::new(10);

    // Enqueue elements
    queue.enqueue(1, Side::Right).unwrap();
    queue.enqueue(2, Side::Right).unwrap();

    // Dequeue elements
    let first = queue.dequeue(Side::Left);
    let second = queue.dequeue(Side::Left);

    println!("First: {:?}", first); // Output: First: Some(1)
    println!("Second: {:?}", second); // Output: Second: Some(2)
}