//! This module implements a circular queue using linked list vertexes. The queue allows adding and removing elements from both ends, maintaining a maximum size.
//! It uses a linked list of vertexes to store the elements, where each vertex can point to its neighboring vertex.
//! This implementation doesn't allow to read elements from the queue, only adding and removing them.
//!
//! # Performance
//! - O(1) for both enqueue and dequeue operations
//! - O(1) for checking if the queue is full or empty
//!  
//! # Implementation Details
//! - The queue is implemented using a doubly linked list where each node (vertex) contains a value and pointers to the next and previous nodes.
//! - The `CircularQueue` struct maintains a cursor pointing to the current vertex, the current size of the queue, and the maximum size allowed.
//! - The queue supports operations to add elements to either end (enqueue) and remove elements from either end (dequeue).
//! - The queue provides methods to check if it is full or empty, and to get the number of elements in the queue.
//! - If the queue is full and an attempt is made to add an element, an error is returned.
//! - If the queue is empty and an attempt is made to remove an element, an error is returned.
//! - The queue can be initialized with a maximum size of 0, which means there is no limit on the number of elements it can hold.
//! 
//! # Usage
//! ```
//! use data_structures::linked_list::circular_queue::CircularQueue;
//! use data_structures::linked_list::circular_queue::Side;
//! 
//! let mut queue = CircularQueue::new(3);
//! 
//! queue.enqueue(1, Side::Right);
//! queue.enqueue(2, Side::Left);
//! queue.enqueue(3, Side::Right);
//! 
//! assert_eq!(queue.dequeue(Side::Left), Some(1));
//! assert_eq!(queue.dequeue(Side::Right), Some(2));
//! assert_eq!(queue.dequeue(Side::Left), Some(3));
//! 
//! assert!(queue.is_empty());
//! ```
//! 
use std::{cell::RefCell, rc::Rc};

use super::vertex::{Vertex, PointerName};


pub enum Side {
    Left,
    Right,
}

impl From<Side> for PointerName {
    fn from(side: Side) -> Self {
        match side {
            Side::Left => PointerName::Left,
            Side::Right => PointerName::Right,
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

    /// Check if the queue is full
    /// # Returns
    /// True if the queue is full, false otherwise
    /// # Example
    /// ```
    /// use data_structures::linked_list::circular_queue::CircularQueue;
    /// use data_structures::linked_list::circular_queue::Side;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// assert_eq!(queue.is_full(), false);
    /// queue.enqueue(1, Side::Right).unwrap();
    /// queue.enqueue(2, Side::Left).unwrap();
    /// queue.enqueue(3, Side::Right).unwrap();
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
    /// use data_structures::linked_list::circular_queue::Side;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// assert_eq!(queue.is_empty(), true);
    /// 
    /// queue.enqueue(1, Side::Right).unwrap();
    /// assert_eq!(queue.is_empty(), false);
    /// 
    /// queue.dequeue(Side::Right).unwrap();
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
    /// use data_structures::linked_list::circular_queue::Side;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// 
    /// assert_eq!(queue.len(), 0);
    /// 
    /// queue.enqueue(1, Side::Right).unwrap();
    /// assert_eq!(queue.len(), 1);
    /// 
    /// queue.enqueue(2, Side::Right).unwrap();
    /// assert_eq!(queue.len(), 2);
    /// 
    /// queue.dequeue(Side::Left).unwrap();
    /// assert_eq!(queue.len(), 1);
    /// 
    /// queue.dequeue(Side::Right).unwrap();
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
    /// use data_structures::linked_list::circular_queue::Side;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// queue.enqueue(1, Side::Right).unwrap();
    /// queue.enqueue(2, Side::Left).unwrap();
    /// queue.enqueue(3, Side::Right).unwrap();
    /// queue.enqueue(4, Side::Right).unwrap_err();
    /// ```
    pub fn enqueue(&mut self, value: T, side: Side) -> Result<(), &'static str> {
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
            cursor_ref.borrow_mut().set_connection(Side::Left.into(), Some(&new_vertex_ptr));
            cursor_ref.borrow_mut().set_connection(Side::Right.into(), Some(&new_vertex_ptr));

            // Adjust the new vertex's pointers. In this case both directions points to the previus added vertex.
            new_vertex_ptr.borrow_mut().set_connection(Side::Right.into(), Some(cursor_ref));
            new_vertex_ptr.borrow_mut().set_connection(Side::Left.into(), Some(cursor_ref));
        
        } else {

            // Get a reference to the current cursor pointer
            let cursor_vertex_ref = self.cursor.as_ref().unwrap();

            // Update the references based on the side
            match side {
                Side::Left => {
                    // Points the right cursor of the new vertex to the cursor's vertex
                    new_vertex_ptr.borrow_mut().set_connection(Side::Right.into(), Some(cursor_vertex_ref));

                    // Points the left cursor of the new vertex to the cursor's left vertex
                    let cursor_left_vertex_ptr = self.cursor.as_ref().unwrap().borrow().get_pointer(Side::Left.into()).unwrap();
                    new_vertex_ptr.borrow_mut().set_connection(Side::Left.into(), Some(&cursor_left_vertex_ptr));

                    // Points the letf vertex's right pointer to the new vertex
                    cursor_left_vertex_ptr.borrow_mut().set_connection(Side::Right.into(),Some(&new_vertex_ptr));

                    // Points the cursor's left pointer to the new vertex
                    cursor_vertex_ref.borrow_mut().set_connection(Side::Left.into(), Some(&new_vertex_ptr));

                },
                Side::Right => {
                    // Points the left cursor of the new vertex to the cursor's vertex
                    new_vertex_ptr.borrow_mut().set_connection(Side::Left.into(), Some(cursor_vertex_ref));

                    // Points the right cursor of the new vertex to the cursor's right vertex
                    let cursor_right_vertex_ptr = self.cursor.as_ref().unwrap().borrow().get_pointer(Side::Right.into()).unwrap();
                    new_vertex_ptr.borrow_mut().set_connection(Side::Right.into(),Some(&cursor_right_vertex_ptr));

                    // Points the right vertex's left pointer to the new vertex
                    cursor_right_vertex_ptr.borrow_mut().set_connection(Side::Left.into(), Some(&new_vertex_ptr));

                    // Points the cursor's right pointer to the new vertex
                    cursor_vertex_ref.borrow_mut().set_connection(Side::Right.into(), Some(&new_vertex_ptr));
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
    /// use data_structures::linked_list::circular_queue::Side;
    /// 
    /// let mut queue: CircularQueue<i32> = CircularQueue::new(3);
    /// 
    /// queue.enqueue(1, Side::Right);
    /// queue.enqueue(2, Side::Right);
    /// 
    /// let removed = queue.dequeue(Side::Left);
    /// assert_eq!(removed, Some(1));
    /// 
    /// let removed = queue.dequeue(Side::Right);
    /// assert_eq!(removed, Some(2));
    /// 
    /// let removed = queue.dequeue(Side::Left);
    /// assert_eq!(removed, None);
    /// ```
    pub fn dequeue(&mut self, side_to_move: Side) -> Option<T> {
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
                other_vertex_ptr.borrow_mut().set_connection(Side::Left.into(), None);
                other_vertex_ptr.borrow_mut().set_connection(Side::Right.into(), None);

                // Set the cursor to the other vertex
                self.cursor = Some(other_vertex_ptr);
            },
            std::cmp::Ordering::Greater => {
                // Get the letf and right vertex reference
                let left_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(Side::Left.into()).unwrap();
                let right_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(Side::Right.into()).unwrap();

                // Points the left vertex's right pointer to the right vertex
                left_vertex_ptr.borrow_mut().set_connection(Side::Right.into(), Some(&right_vertex_ptr));

                // Points the right vertex's left pointer to the left vertex
                right_vertex_ptr.borrow_mut().set_connection(Side::Left.into(), Some(&left_vertex_ptr));

                // Update the cursor based on the side
                match side_to_move {
                    Side::Left => {
                        // Set the cursor to the left vertex
                        self.cursor = Some(left_vertex_ptr);
                    },
                    Side::Right => {
                        // Set the cursor to the right vertex
                        self.cursor = Some(right_vertex_ptr);
                    }
                }
            },
            std::cmp::Ordering::Less => {
                // In this case we don't have to do anything.
            }
        }

        /*
        if self.len() == 2 {

            // Get the other vertex that will remain in the queue
            let other_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(side_to_move).unwrap();

            // Points the other vertex's left and right pointers to None
            other_vertex_ptr.borrow_mut().set_connection(None, Side::Left);
            other_vertex_ptr.borrow_mut().set_connection(None, Side::Right);

            // Set the cursor to the other vertex
            self.cursor = Some(other_vertex_ptr);

        }else if self.len() > 2 {
            // Get the letf and right vertex reference
            let left_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(Side::Left).unwrap();
            let right_vertex_ptr = vertex_to_remove_ref.borrow().get_pointer(Side::Right).unwrap();

            // Points the left vertex's right pointer to the right vertex
            left_vertex_ptr.borrow_mut().set_connection(Some(&right_vertex_ptr), Side::Right);

            // Points the right vertex's left pointer to the left vertex
            right_vertex_ptr.borrow_mut().set_connection(Some(&left_vertex_ptr), Side::Left);

            // Update the cursor based on the side
            match side_to_move {
                Side::Left => {
                    // Set the cursor to the left vertex
                    self.cursor = Some(left_vertex_ptr);
                },
                Side::Right => {
                    // Set the cursor to the right vertex
                    self.cursor = Some(right_vertex_ptr);
                }
            }
        }*/

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
            queue.enqueue(i, Side::Left).unwrap();
        }

        assert_eq!(queue.len(), 10);

        let removed = queue.dequeue(Side::Right);
        assert_eq!(removed, Some(0));

        let removed = queue.dequeue(Side::Right);
        assert_eq!(removed, Some(1));

        let removed = queue.dequeue(Side::Left);
        assert_eq!(removed, Some(2));

        let removed = queue.dequeue(Side::Left);
        assert_eq!(removed, Some(9));
    }

    #[test]
    fn test_queue() {
        let mut queue: super::CircularQueue<i32> = super::CircularQueue::new(10);

        assert!(queue.is_empty());

        for i in 0..10 {
            queue.enqueue(i, Side::Left).unwrap();
        }

        assert_eq!(queue.len(), 10);

        let resul = queue.enqueue(10, Side::Left).unwrap_err();
        assert_eq!(resul, "Queue is full");

        let removed = queue.dequeue(Side::Right);
        assert_eq!(removed, Some(0));

        let removed = queue.dequeue(Side::Right);
        assert_eq!(removed, Some(1));

        let removed = queue.dequeue(Side::Left);
        assert_eq!(removed, Some(2));

        let removed = queue.dequeue(Side::Left);
        assert_eq!(removed, Some(9));
    }

    #[test]
    fn test_memory_leak() {
        let mut queue: CircularQueue<i32> = CircularQueue::new(10);

        let mut vertexes = Vec::new();

        for i in 0..10 {
            let vertex = Vertex::new(i);
            vertexes.push(vertex.clone());
            queue.enqueue(i, Side::Left).unwrap();
        }

        assert_eq!(queue.len(), 10);

        for _ in 0..10 {
            queue.dequeue(Side::Right);
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
}