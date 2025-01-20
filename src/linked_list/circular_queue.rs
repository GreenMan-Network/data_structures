//! This module implements a circular queue using linked list blocks. The queue allows adding and removing elements from both ends, maintaining a maximum size.
//! It uses a linked list of blocks to store the elements, where each block can point to its neighboring blocks.
//! This implementation doesn't allow to read elements from the queue, only adding and removing them.
//!
//! # Performance
//! - O(1) for both enqueue and dequeue operations
//! - O(1) for checking if the queue is full or empty
//!  
//! # Implementation Details
//! - The queue is implemented using a doubly linked list where each node (block) contains a value and pointers to the next and previous nodes.
//! - The `CircularQueue` struct maintains a cursor pointing to the current block, the current size of the queue, and the maximum size allowed.
//! - The queue supports operations to add elements to either end (enqueue) and remove elements from either end (dequeue).
//! - The queue provides methods to check if it is full or empty, and to get the number of elements in the queue.
//! - If the queue is full and an attempt is made to add an element, an error is returned.
//! - If the queue is empty and an attempt is made to remove an element, an error is returned.
//! - The queue can be initialized with a maximum size of 0, which means there is no limit on the number of elements it can hold.
//! 
//! # Usage
//! ```
//! use data_structures::linked_list::circular_queue::CircularQueue;
//! use data_structures::linked_list::block::Side;
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

use super::block::{Block, Side};

/// Struct representing a circular queue using linked list blocks
/// This queue allows adding and removing elements from both ends.
/// The queue maintains a maximum size, and will return an error if an attempt is made to add an element when the queue is full.
/// The queue can be initialized with a maximum size of 0, which means there is no limit on the number of elements it can hold.
/// The queue uses a linked list of blocks to store the elements, where each block can point to its neighboring blocks.
/// The queue supports operations to add elements to either end and remove elements from either end.
/// The queue also provides methods to check if it is full or empty, and to get the number of elements in the queue.
/// 
#[derive(Debug)]
pub struct CircularQueue<T> {
    cursor: Option<Rc<RefCell<Block<T>>>>,

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
    /// use data_structures::linked_list::block::Side;
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
    /// use data_structures::linked_list::block::Side;
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
    /// use data_structures::linked_list::block::Side;
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
    /// use data_structures::linked_list::block::Side;
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
        
        // Create new block
        let new_block_ptr = Block::new(value);

        // Test if the queue is not empty
        if self.is_empty(){
           
            // If the queue is empty, set the cursor to the new block
            self.cursor = Some(new_block_ptr);

        } else if self.len() == 1 {

            // Get a reference to the current cursor pointer
            let cursor_ref = self.cursor.as_ref().unwrap();

            // Insert the new block. In this case, both directions points to the new block
            cursor_ref.borrow_mut().set_pointer(Some(&new_block_ptr), Side::Left);
            cursor_ref.borrow_mut().set_pointer(Some(&new_block_ptr), Side::Right);

            // Adjust the new block's pointers. In this case both directions points to the previus added block.
            new_block_ptr.borrow_mut().set_pointer(Some(cursor_ref), Side::Right);
            new_block_ptr.borrow_mut().set_pointer(Some(cursor_ref), Side::Left);
        
        } else {

            // Get a reference to the current cursor pointer
            let cursor_block_ref = self.cursor.as_ref().unwrap();

            // Update the references based on the side
            match side {
                Side::Left => {
                    // Points the right cursor of the new block to the cursor's block
                    new_block_ptr.borrow_mut().set_pointer(Some(cursor_block_ref), Side::Right);

                    // Points the left cursor of the new block to the cursor's left block
                    let cursor_left_block_ptr = self.cursor.as_ref().unwrap().borrow().get_pointer(Side::Left).unwrap();
                    new_block_ptr.borrow_mut().set_pointer(Some(&cursor_left_block_ptr), Side::Left);

                    // Points the letf block's right pointer to the new block
                    cursor_left_block_ptr.borrow_mut().set_pointer(Some(&new_block_ptr), Side::Right);

                    // Points the cursor's left pointer to the new block
                    cursor_block_ref.borrow_mut().set_pointer(Some(&new_block_ptr), Side::Left);

                },
                Side::Right => {
                    // Points the left cursor of the new block to the cursor's block
                    new_block_ptr.borrow_mut().set_pointer(Some(cursor_block_ref), Side::Left);

                    // Points the right cursor of the new block to the cursor's right block
                    let cursor_right_block_ptr = self.cursor.as_ref().unwrap().borrow().get_pointer(Side::Right).unwrap();
                    new_block_ptr.borrow_mut().set_pointer(Some(&cursor_right_block_ptr), Side::Right);

                    // Points the right block's left pointer to the new block
                    cursor_right_block_ptr.borrow_mut().set_pointer(Some(&new_block_ptr), Side::Left);

                    // Points the cursor's right pointer to the new block
                    cursor_block_ref.borrow_mut().set_pointer(Some(&new_block_ptr), Side::Right);
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
    /// use data_structures::linked_list::block::Side;
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

        // Get the current cursor Block pointer and erase the cursor
        let block_to_remove_ref = self.cursor.take().unwrap();


        match self.len().cmp(&2) {
            std::cmp::Ordering::Equal => {
                // Get the other block that will remain in the queue
                let other_block_ptr = block_to_remove_ref.borrow().get_pointer(side_to_move).unwrap();

                // Points the other block's left and right pointers to None
                other_block_ptr.borrow_mut().set_pointer(None, Side::Left);
                other_block_ptr.borrow_mut().set_pointer(None, Side::Right);

                // Set the cursor to the other block
                self.cursor = Some(other_block_ptr);
            },
            std::cmp::Ordering::Greater => {
                // Get the letf and right block reference
                let left_block_ptr = block_to_remove_ref.borrow().get_pointer(Side::Left).unwrap();
                let right_block_ptr = block_to_remove_ref.borrow().get_pointer(Side::Right).unwrap();

                // Points the left block's right pointer to the right block
                left_block_ptr.borrow_mut().set_pointer(Some(&right_block_ptr), Side::Right);

                // Points the right block's left pointer to the left block
                right_block_ptr.borrow_mut().set_pointer(Some(&left_block_ptr), Side::Left);

                // Update the cursor based on the side
                match side_to_move {
                    Side::Left => {
                        // Set the cursor to the left block
                        self.cursor = Some(left_block_ptr);
                    },
                    Side::Right => {
                        // Set the cursor to the right block
                        self.cursor = Some(right_block_ptr);
                    }
                }
            },
            std::cmp::Ordering::Less => {
                // In this case we don't have to do anything.
            }
        }

        /*
        if self.len() == 2 {

            // Get the other block that will remain in the queue
            let other_block_ptr = block_to_remove_ref.borrow().get_pointer(side_to_move).unwrap();

            // Points the other block's left and right pointers to None
            other_block_ptr.borrow_mut().set_pointer(None, Side::Left);
            other_block_ptr.borrow_mut().set_pointer(None, Side::Right);

            // Set the cursor to the other block
            self.cursor = Some(other_block_ptr);

        }else if self.len() > 2 {
            // Get the letf and right block reference
            let left_block_ptr = block_to_remove_ref.borrow().get_pointer(Side::Left).unwrap();
            let right_block_ptr = block_to_remove_ref.borrow().get_pointer(Side::Right).unwrap();

            // Points the left block's right pointer to the right block
            left_block_ptr.borrow_mut().set_pointer(Some(&right_block_ptr), Side::Right);

            // Points the right block's left pointer to the left block
            right_block_ptr.borrow_mut().set_pointer(Some(&left_block_ptr), Side::Left);

            // Update the cursor based on the side
            match side_to_move {
                Side::Left => {
                    // Set the cursor to the left block
                    self.cursor = Some(left_block_ptr);
                },
                Side::Right => {
                    // Set the cursor to the right block
                    self.cursor = Some(right_block_ptr);
                }
            }
        }*/

        self.size -= 1;
        
        // Get data from block and discard the block
        let data = block_to_remove_ref.borrow_mut().get_data();
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

        let mut blocks = Vec::new();

        for i in 0..10 {
            let block = Block::new(i);
            blocks.push(block.clone());
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

        // Check the reference count of each block
        for block in blocks {
            assert_eq!(Rc::strong_count(&block), 1);
        }
    }
}