//! This module implements a circular queue using linked list vertexes. The queue allows adding and removing elements from both ends, maintaining a maximum size.
//! It uses a linked list of vertexes to store the elements, where each vertex can point to its neighboring vertex.
//! This implementation doesn't allow to read elements from the queue, only adding and removing them.
//!
//! # Performance
//! - O(1) for both insert and remove operations
//! - O(1) for checking if the queue is full or empty
//!  
//! # Implementation Details
//! - The queue is implemented using a doubly linked list where each node (vertex) contains a value and pointers to the next and previous nodes.
//! - The `CircularQueue` struct maintains a cursor pointing to the current vertex, the current size of the queue, and the maximum size allowed.
//! - The queue supports operations to add elements to either end (insert) and remove elements from either end (remove).
//! - The queue provides methods to check if it is full or empty, and to get the number of elements in the queue.
//! - If the queue is full and an attempt is made to add an element, an error is returned.
//! - If the queue is empty and an attempt is made to remove an element, an error is returned.
//! - The queue can be initialized with a maximum size of 0, which means there is no limit on the number of elements it can hold.
//! 
//! # Usage
//! ```
//! use data_structures::linked_list::circular_queue::CircularQueue;
//! use data_structures::linked_list::circular_queue::Direction;
//! 
//! let mut queue = CircularQueue::new(3);
//! 
//! queue.insert(1, Direction::Right);
//! queue.insert(2, Direction::Left);
//! queue.insert(3, Direction::Right);
//! 
//! assert_eq!(queue.remove(Direction::Left), Some(1));
//! assert_eq!(queue.remove(Direction::Right), Some(2));
//! assert_eq!(queue.remove(Direction::Left), Some(3));
//! 
//! assert!(queue.is_empty());
//! ```
//! 
use std::{cell::RefCell, rc::Rc};

use super::vertex::{Vertex, PointerName};


pub enum Direction {
    Left,
    Right,
}

impl From<Direction> for PointerName {
    fn from(side: Direction) -> Self {
        match side {
            Direction::Left => PointerName::Left,
            Direction::Right => PointerName::Right,
        }
    }
}
/// Struct representing a circular queue using linked list vertexes
/// This queue allows adding and removing elements from both ends.
/// The queue maintains a maximum size, and will return an error if an attempt is made to add an element when the queue is full.
/// The queue can be initialized with a maximum size of 0, which means there is no limit on the number of elements it can hold.
/// The queue uses a linked list of vertexes to store the elements, where each vertex can point to its neighboring vertexes.
/// The queue supports operations to add elements to either end and remove elements from either end.
/// The queue also provides methods to check if it is full or empty, and to get the number of elements in the queue.
/// 
#[derive(Debug)]
pub struct CircularQueue<T> {
    cursor: Option<Rc<RefCell<Vertex<T>>>>,

    size: usize,
    max_size: usize,
}

impl<T> CircularQueue<T>{
    /// Create a new CircularQueue with the given maximum size
    ///
    /// # Arguments
    /// * `max_size`: The maximum number of elements the queue can hold. If 0, there is no size limit.
    /// 
    /// # Returns
    /// A new CircularQueue instance
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// assert_eq!(queue.is_empty(), true);
    /// ```
    /// 
    pub fn new(max_size: usize) -> Self {
        CircularQueue {
            cursor: None,
            size: 0,
            max_size,
        }
    }


    /// Get the maximum size of the queue
    /// # Returns
    /// The maximum size of the queue
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// 
    /// let queue: CircularQueue<i32> = CircularQueue::new(3);
    /// 
    /// assert_eq!(queue.max_size(), 3);
    /// ```
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Set a new maximum size for the queue
    /// # Arguments
    /// * `max_size`: The new maximum size for the queue
    /// # Returns
    /// Result<(), &'static str>
    /// Ok if the new maximum size is set successfully, Err if the new maximum size is less than the current size
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Direction;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(0);
    /// 
    /// queue.insert(1, Direction::Right);
    /// queue.insert(2, Direction::Right);
    /// queue.insert(3, Direction::Right);
    /// 
    /// assert_eq!(queue.set_max_size(2), Err("New max size is less than current size"));
    /// assert_eq!(queue.set_max_size(3), Ok(()));
    /// 
    /// assert_eq!(queue.insert(4, Direction::Right), Err("Queue is full"));
    /// ```
    pub fn set_max_size(&mut self, max_size: usize) -> Result<(), &'static str>{
        if self.len() > max_size {
            Err("New max size is less than current size")
        } else {
            self.max_size = max_size;
            Ok(())
        }
    }

    /// Check if the queue is full
    /// # Returns
    /// True if the queue is full, false otherwise
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Direction;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// assert_eq!(queue.is_full(), false);
    /// queue.insert(1, Direction::Right).unwrap();
    /// queue.insert(2, Direction::Left).unwrap();
    /// queue.insert(3, Direction::Right).unwrap();
    /// assert_eq!(queue.is_full(), true);
    /// ```
    /// 
    pub fn is_full(&self) -> bool {
        if self.max_size == 0 {
            return false;
        }
        self.size == self.max_size
    }

    /// Check if the queue is empty
    /// # Returns
    /// True if the queue is empty, false otherwise
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Direction;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// assert_eq!(queue.is_empty(), true);
    /// 
    /// queue.insert(1, Direction::Right).unwrap();
    /// assert_eq!(queue.is_empty(), false);
    /// 
    /// queue.remove(Direction::Right).unwrap();
    /// assert_eq!(queue.is_empty(), true);
    /// ```
    /// 
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Get the number of elements in the queue
    /// # Returns
    /// The number of elements in the queue
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Direction;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// 
    /// assert_eq!(queue.len(), 0);
    /// 
    /// queue.insert(1, Direction::Right).unwrap();
    /// assert_eq!(queue.len(), 1);
    /// 
    /// queue.insert(2, Direction::Right).unwrap();
    /// assert_eq!(queue.len(), 2);
    /// 
    /// queue.remove(Direction::Left).unwrap();
    /// assert_eq!(queue.len(), 1);
    /// 
    /// queue.remove(Direction::Right).unwrap();
    /// assert_eq!(queue.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Add an element to the queue
    /// # Arguments
    /// * `value`: The value to be added to the queue
    /// * `side`: The side to add the element to (Left or Right)
    /// # Returns
    /// Result<(), &'static str>
    /// Ok if the element was added successfully, Err if the queue is full
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Direction;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// queue.insert(1, Direction::Right).unwrap();
    /// queue.insert(2, Direction::Left).unwrap();
    /// queue.insert(3, Direction::Right).unwrap();
    /// queue.insert(4, Direction::Right).unwrap_err();
    /// ```
    pub fn insert(&mut self, value: T, side: Direction) -> Result<(), &'static str> {
        // Returns an error if the queue is full
        if self.is_full() {
            return Err("Queue is full");
        }
        
        // Create new vertex
        let new_vertex_ptr = Vertex::new(value);

        // Test if the queue is not empty
        if self.is_empty(){
           
            // If the queue is empty, set the cursor to the new vertex
            self.cursor = Some(new_vertex_ptr);

        } else if self.len() == 1 {

            // Get a reference to the current cursor pointer
            let cursor_ref = self.cursor.as_ref().unwrap();

            // Insert the new vertex. In this case, both directions points to the new vertex
            cursor_ref.borrow_mut().set_connection(Direction::Left.into(), Some(&new_vertex_ptr));
            cursor_ref.borrow_mut().set_connection(Direction::Right.into(), Some(&new_vertex_ptr));

            // Adjust the new vertex's pointers. In this case both directions points to the previus added vertex.
            new_vertex_ptr.borrow_mut().set_connection(Direction::Right.into(), Some(cursor_ref));
            new_vertex_ptr.borrow_mut().set_connection(Direction::Left.into(), Some(cursor_ref));
        
        } else {

            // Get a reference to the current cursor pointer
            let cursor_vertex_ref = self.cursor.as_ref().unwrap();

            // Update the references based on the side
            match side {
                Direction::Left => {
                    // Points the right cursor of the new vertex to the cursor's vertex
                    new_vertex_ptr.borrow_mut().set_connection(Direction::Right.into(), Some(cursor_vertex_ref));

                    // Points the left cursor of the new vertex to the cursor's left vertex
                    let cursor_left_vertex_ptr = self.cursor.as_ref().unwrap().borrow().get_pointer(Direction::Left.into()).unwrap();
                    new_vertex_ptr.borrow_mut().set_connection(Direction::Left.into(), Some(&cursor_left_vertex_ptr));

                    // Points the letf vertex's right pointer to the new vertex
                    cursor_left_vertex_ptr.borrow_mut().set_connection(Direction::Right.into(),Some(&new_vertex_ptr));

                    // Points the cursor's left pointer to the new vertex
                    cursor_vertex_ref.borrow_mut().set_connection(Direction::Left.into(), Some(&new_vertex_ptr));

                },
                Direction::Right => {
                    // Points the left cursor of the new vertex to the cursor's vertex
                    new_vertex_ptr.borrow_mut().set_connection(Direction::Left.into(), Some(cursor_vertex_ref));

                    // Points the right cursor of the new vertex to the cursor's right vertex
                    let cursor_right_vertex_ptr = self.cursor.as_ref().unwrap().borrow().get_pointer(Direction::Right.into()).unwrap();
                    new_vertex_ptr.borrow_mut().set_connection(Direction::Right.into(),Some(&cursor_right_vertex_ptr));

                    // Points the right vertex's left pointer to the new vertex
                    cursor_right_vertex_ptr.borrow_mut().set_connection(Direction::Left.into(), Some(&new_vertex_ptr));

                    // Points the cursor's right pointer to the new vertex
                    cursor_vertex_ref.borrow_mut().set_connection(Direction::Right.into(), Some(&new_vertex_ptr));
                }
            }

        }

        self.size += 1;

        Ok(())
    }

    /// Remove and return an element from the queue
    /// # Arguments
    /// * `side_to_move`: The side to move the cursor after removing the data (Left or Right)
    /// 
    /// # Returns
    /// The removed element, or None if the queue is empty
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Direction;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// 
    /// queue.insert(1, Direction::Right);
    /// queue.insert(2, Direction::Right);
    /// 
    /// let removed = queue.remove(Direction::Left);
    /// assert_eq!(removed, Some(1));
    /// 
    /// let removed = queue.remove(Direction::Right);
    /// assert_eq!(removed, Some(2));
    /// 
    /// let removed = queue.remove(Direction::Left);
    /// assert_eq!(removed, None);
    /// ```
    pub fn remove(&mut self, side_to_move: Direction) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // Get the current cursor Vertex pointer and erase the cursor
        let vertex_to_remove_ref = self.cursor.take().unwrap();


        match self.len().cmp(&2) {
            std::cmp::Ordering::Equal => {
                // Get the other vertex that will remain in the queue
                let other_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(side_to_move.into()).unwrap();

                // Points the other vertex's left and right pointers to None
                other_vertex_ptr.borrow_mut().set_connection(Direction::Left.into(), None);
                other_vertex_ptr.borrow_mut().set_connection(Direction::Right.into(), None);

                // Set the cursor to the other vertex
                self.cursor = Some(other_vertex_ptr);
            },
            std::cmp::Ordering::Greater => {
                // Get the letf and right vertex reference
                let left_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(Direction::Left.into()).unwrap();
                let right_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(Direction::Right.into()).unwrap();

                // Points the left vertex's right pointer to the right vertex
                left_vertex_ptr.borrow_mut().set_connection(Direction::Right.into(), Some(&right_vertex_ptr));

                // Points the right vertex's left pointer to the left vertex
                right_vertex_ptr.borrow_mut().set_connection(Direction::Left.into(), Some(&left_vertex_ptr));

                // Update the cursor based on the side
                match side_to_move {
                    Direction::Left => {
                        // Set the cursor to the left vertex
                        self.cursor = Some(left_vertex_ptr);
                    },
                    Direction::Right => {
                        // Set the cursor to the right vertex
                        self.cursor = Some(right_vertex_ptr);
                    }
                }
            },
            std::cmp::Ordering::Less => {
                // In this case we don't have to do anything.
            }
        }

        self.size -= 1;
        
        // Get data from vertex and discard the vertex
        let data = vertex_to_remove_ref.borrow_mut().clear();
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_no_size_limit() {
        let mut queue: super::CircularQueue<i32> = super::CircularQueue::new(0);

        assert!(queue.is_empty());

        for i in 0..10 {
            queue.insert(i, Direction::Left).unwrap();
        }

        assert_eq!(queue.len(), 10);

        let removed = queue.remove(Direction::Right);
        assert_eq!(removed, Some(0));

        let removed = queue.remove(Direction::Right);
        assert_eq!(removed, Some(1));

        let removed = queue.remove(Direction::Left);
        assert_eq!(removed, Some(2));

        let removed = queue.remove(Direction::Left);
        assert_eq!(removed, Some(9));
    }

    #[test]
    fn test_queue() {
        let mut queue: super::CircularQueue<i32> = super::CircularQueue::new(10);

        assert!(queue.is_empty());

        for i in 0..10 {
            queue.insert(i, Direction::Left).unwrap();
        }

        assert_eq!(queue.len(), 10);

        let resul = queue.insert(10, Direction::Left).unwrap_err();
        assert_eq!(resul, "Queue is full");

        let removed = queue.remove(Direction::Right);
        assert_eq!(removed, Some(0));

        let removed = queue.remove(Direction::Right);
        assert_eq!(removed, Some(1));

        let removed = queue.remove(Direction::Left);
        assert_eq!(removed, Some(2));

        let removed = queue.remove(Direction::Left);
        assert_eq!(removed, Some(9));
    }

    #[test]
    fn test_memory_leak() {
        let mut queue: CircularQueue<i32> = CircularQueue::new(10);

        let mut vertexes = Vec::new();

        for i in 0..10 {
            let vertex = Vertex::new(i);
            vertexes.push(vertex.clone());
            queue.insert(i, Direction::Left).unwrap();
        }

        assert_eq!(queue.len(), 10);

        for _ in 0..10 {
            queue.remove(Direction::Right);
        }

        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());

        // Check if all references are dropped
        assert!(queue.cursor.is_none());

        // Check the reference count of each vertex
        for vertex in vertexes {
            assert_eq!(Rc::strong_count(&vertex), 1);
        }
    }

    #[test]
    fn test_circular_queue_stress() {
        use std::time::Instant;

        // Define the maximum size of the queue and the number of iterations
        let max_size = 10_000;
        let iterations = 1_000_000;

        // Create a new CircularQueue
        let mut queue: CircularQueue<u32> = CircularQueue::new(max_size);

        // Record the start time
        let start_time = Instant::now();

        // Enqueue and dequeue in a loop
        for i in 0..iterations {
            let value = i as u32; // Use modulo to keep values within range

            // Enqueue the value
            let result = queue.insert(value, Direction::Right);

            assert!(result.is_ok());

            // Dequeue when the queue reaches its max size
            if queue.is_full() {
                let dequeued_value = queue.remove(Direction::Left).unwrap();

                // Validate that the dequeued value is correct
                assert_eq!(dequeued_value, value - (max_size as u32 - 1));
            }
        }

        // Ensure the queue is empty after all iterations
        while !queue.is_empty() {
            queue.remove(Direction::Left);
        }

        assert!(queue.is_empty());

        // Record the end time
        let duration = start_time.elapsed();
        println!("Stress test completed in {:?}", duration);
    }

    #[test]
    fn test_vec_as_circular_queue_stress() {
        use std::time::Instant;

        // Define the maximum size of the queue and the number of iterations
        let max_size = 10_000;
        let iterations = 1_000_000;

        // Create a Vec to simulate a circular queue
        let mut vec_queue: Vec<u32> = Vec::new();

        // Record the start time
        let start_time = Instant::now();

        // Enqueue and dequeue in a loop
        for i in 0..iterations {
            let value = i as u32;

            // Enqueue the value
            vec_queue.push(value);

            // Dequeue when the queue reaches its max size
            if vec_queue.len() == max_size {
                // Remove the oldest element (simulate FIFO)
                let dequeued_value = vec_queue.remove(0);

                // Calculate the expected value, accounting for wrapping
                let expected_value = i as u32 - (max_size as u32 - 1);
                assert_eq!(dequeued_value, expected_value);
            }
        }

        // Ensure the queue is empty after all iterations
        vec_queue.clear();
        assert!(vec_queue.is_empty());

        // Record the end time
        let duration = start_time.elapsed();
        println!("Vec stress test completed in {:?}", duration);
    }
}