# Data Structures Library

This library contains a collection of data structures implemented in Rust. It is designed to be freely used and serves as a laboratory for understanding the usage of references in Rust. The library exclusively uses safe references to ensure memory safety and prevent common issues such as dangling pointers and memory leaks.

## Purpose

The primary purpose of this library is to provide a set of data structures that can be used in various applications.

Additionally, it serves as a learning tool for understanding how to work with references in Rust, including the use of `Rc`, `RefCell`, and `Weak` references. These structures emphasize safe memory management while maintaining flexibility and performance.

## Data Structures

### 1. Vertex

A vertex is a fundamental building block for linked data structures. Each vertex contains data and pointers to other vertices. This implementation is highly flexible and supports complex connections such as bi-directional links or custom relationships.

#### **Use Cases**
- Linked lists (single and doubly linked)
- Circular linked lists
- Graphs and networks

#### **Performance**
- **Access:** O(1)
- **Insertion:** O(1)
- **Deletion:** O(1)

#### **Implementation Details**
- Each vertex is encapsulated in `Rc<RefCell<Vertex<T>>>` for multiple ownership and interior mutability.
- Connections between vertices are stored in a `HashMap`, allowing flexibility in naming relationships (e.g., `PointerName::Left`, `PointerName::Right`, or custom names).
- Weak references (`Weak`) are used for self-referencing to prevent memory leaks caused by reference cycles.

### 2. Circular Queue

A circular queue is a linear data structure that follows the FIFO (First In, First Out) principle but connects the end of the queue back to the front, forming a circle. This implementation uses a doubly linked list of vertices, where each vertex can point to its neighboring vertices.

#### **Use Cases**
- Task scheduling
- Buffer management
- Round-robin scheduling

#### **Performance**
- **Insertion:** O(1)
- **Removal:** O(1)
- **Check full/empty status:** O(1)

#### **Implementation Details**
- Implemented using a doubly linked list for efficient insertion and removal from both ends.
- Each vertex is managed using `Rc<RefCell<Vertex<T>>>`, enabling shared ownership and interior mutability.
- The circular queue maintains a cursor pointing to the current vertex, supporting bi-directional traversal.
- Handles resizing dynamically with configurable maximum size (including unlimited size when set to zero).

### 3. FIFO (First In, First Out)

The FIFO is a specialized queue implemented using the `CircularQueue`. It enforces the FIFO principle, where elements are inserted at one end and removed from the other.

#### **Use Cases**
- Job queues
- Message processing
- Buffering operations

#### **Performance**
- **Push:** O(1)
- **Pop:** O(1)
- **Check full/empty status:** O(1)

#### **Implementation Details**
- Built as a lightweight wrapper around the `CircularQueue`.
- Provides a simplified API (`push` and `pop`) for FIFO-specific operations.
- Supports dynamic resizing and efficient memory reuse.

## Safe References

This library exclusively uses safe references to manage memory. The following types are employed to ensure safety:

- **`Rc` (Reference Counted):** Enables multiple ownership of data while automatically deallocating memory when no owners remain.
- **`RefCell`:** Allows interior mutability, enabling mutation of data even when the `RefCell` itself is immutable.
- **`Weak`:** Provides non-owning references, preventing reference cycles and enabling safe self-referencing.

## Design Philosophy

1. **Memory Safety:** By leveraging Rust's ownership model and safe references, the library ensures that memory-related bugs like use-after-free or double-free are impossible.
2. **Flexibility:** The use of generic types and modular design allows these structures to be applied in a variety of contexts.
3. **Performance:** Operations are designed to achieve constant time complexity (O(1)) where possible, ensuring efficiency even under heavy workloads.

## Getting Started

To use this library, add the module to your Rust project. You can find the core modules under the `linked_list` namespace:

```rust
pub mod linked_list {
    pub mod vertex;
    pub mod circular_queue;
    pub mod fifo;
}
```

### Example Usage

```rust
use data_structures::linked_list::fifo::FIFO;

let mut fifo = FIFO::new(5);

fifo.push(1).unwrap();
fifo.push(2).unwrap();
fifo.push(3).unwrap();

assert_eq!(fifo.pop(), Some(1));
assert_eq!(fifo.pop(), Some(2));
assert_eq!(fifo.is_empty(), false);
assert_eq!(fifo.pop(), Some(3));
assert_eq!(fifo.is_empty(), true);
```

## Testing

The library includes extensive unit tests to ensure correctness and performance. Run the tests using:

```bash
cargo test
```
[![Tests](https://github.com/GreenMan-Network/data_structures/actions/workflows/tests.yml/badge.svg)](https://github.com/GreenMan-Network/data_structures/actions/workflows/tests.yml)


## License

This library is open-source and available under the MIT license. Feel free to use and modify it for your projects.

## References

* [noahgift/rust-new-project-template](https://github.com/noahgift/rust-new-project-template.git)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [hello-rust](https://github.com/nogibjj/hello-rust)
