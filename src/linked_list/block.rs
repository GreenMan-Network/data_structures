//! This module defines a Block struct that represents a block in a linked list.
//! It includes methods for creating a new block, accessing and modifying the data, and managing pointers to the next and previous blocks.
//! 
//! # Performance
//! - Accessing the data in a block is O(1).
//! - Updating the pointers to the next and previous blocks is O(1).
//! - Creating a new block is O(1).
//! 
//! # Usage
//! ```
//! use data_structures::linked_list::block::{Block, Side};
//! 
//! let block_ptr = Block::new(10);
//! assert_eq!(*block_ptr.borrow().read_data(), Some(10));
//! ```
use std::{cell::RefCell, rc::{Rc, Weak}};

/// Direction of the pointer inside the Block
/// 
/// This enum is used to specify the direction of the pointer in a block of a doubly linked list.
/// It helps in identifying whether the pointer is pointing to the next block (Right) or the previous block (Left).
pub enum Side {
    Left,
    Right,
}

/// A block in a linked list
/// Each block contains data and pointers to the next and previous blocks
/// # Type Parameters
/// * `T`: The type of data to be stored in the block
/// * right: Pointer to the right block in the list
/// * left: Pointer to the left block in the list
#[derive(Debug)]
pub struct Block<T> {
    data: Option<T>,
    block_ref: Option<Weak<RefCell<Block<T>>>>, // reference to the block itself
    right: Option<Rc<RefCell<Block<T>>>>,    // points to the next block
    left: Option<Rc<RefCell<Block<T>>>>,    // points to the previous block
}

impl<T> Block<T> {
    /// Create a new block with the given data and return a pointer to it
    /// # Arguments
    /// * `data`: The data to be stored in the block. Tghe block takes ownership of the data.
    /// 
    /// # Returns
    /// A pointer to the newly created block.
    /// A copy of the pointer is keep in the block itself, so if exists while the block is alive
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// 
    /// let block_ptr = Block::new(10);
    /// assert_eq!(*block_ptr.borrow().read_data(), Some(10));
    /// ```
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        // Create new empty block
        let new_block_ptr = Rc::new(RefCell::new(Block {
            data: None,
            block_ref: None, // Temporariamente None
            right: None,
            left: None,
        }));

        // Set the block_ref to point to itself
        new_block_ptr.borrow_mut().block_ref = Some(Rc::downgrade(&new_block_ptr));

        // Set the data
        new_block_ptr.borrow_mut().data = Some(data);

        new_block_ptr
    }

    /// Get a reference to the block itself keeping the reference count
    /// # Returns
    /// A reference to the block itself
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// use std::rc::Rc;
    /// 
    /// let block_ptr = Block::new(10);
    /// let new_block_ptr = block_ptr.borrow().get_reference();
    /// assert_eq!(Rc::strong_count(&block_ptr), 2);
    /// assert_eq!(Rc::strong_count(&new_block_ptr), 2);
    /// ```
    pub fn get_reference(&self) -> Rc<RefCell<Block<T>>> {
        self.block_ref.as_ref().and_then(|weak_ref| weak_ref.upgrade()).unwrap()
    }

    /// Get a reference to the data
    /// Useful for read-only access
    /// 
    /// # Returns
    /// A reference to the data
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// let block_ptr = Block::new(10);
    /// assert_eq!(block_ptr.borrow().read_data().unwrap(), 10);
    /// ```
    /// 
    pub fn read_data(&self) -> &Option<T> {
        &self.data
    }

    /// Set the data of the block and return the old data
    /// # Arguments
    /// * `data`: The new data to be set in the block
    /// # Returns
    /// The old data of the block
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// 
    /// let mut block_ptr = Block::new(10);
    /// 
    /// assert_eq!(block_ptr.borrow_mut().set_data(20), Some(10));
    /// assert_eq!(block_ptr.borrow().read_data().unwrap(), 20);
    /// ```
    pub fn set_data(&mut self, data: T) -> Option<T> {
        self.data.replace(data)
    }

    /// Comsumes the block and returns the data
    /// Returns the data and erase the pointer to the next and previous blocks
    /// 
    /// # Returns
    /// The data contained in the block
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// let block_ptr = Block::new(10);
    /// assert_eq!(block_ptr.borrow_mut().get_data(), Some(10));
    /// ```
    /// 
    pub fn get_data(&mut self) -> Option<T> {
        self.right = None;
        self.left = None;
        self.data.take()
    }

    /// Borrows the right pointer.
    /// As a pattern, this method returns an Option of a Weak pointer to the right block.
    /// Strong references could be created from the Weak pointer or the Block if needed.
    /// 
    /// # Returns
    /// A reference to the right pointer
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// use data_structures::linked_list::block::Side;
    /// 
    /// let block_ptr = Block::new(10);
    /// assert_eq!(block_ptr.borrow().get_pointer(Side::Left).is_none(), true);
    /// assert_eq!(block_ptr.borrow().get_pointer(Side::Right).is_none(), true);
    /// ```
    pub fn get_pointer(&self, side: Side) -> Option<Rc<RefCell<Block<T>>>> {
        match side {
            Side::Left => self.left.clone(),
            Side::Right => self.right.clone(),
        }
    }

    /// Replace the right pointer with a new block and return the old one
    /// # Arguments
    /// * `new_block_ptr`: The new block to be set as the right pointer
    /// # Returns
    /// The old right block pointer
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// use data_structures::linked_list::block::Side;
    /// 
    /// let block1_ptr = Block::new(10);
    /// let block2_ptr = Block::new(20);
    /// 
    /// // Set the right pointer of block1 to block2
    /// let prev_block_ptr = block1_ptr.borrow_mut().set_pointer(Some(&block2_ptr), Side::Right);
    /// assert_eq!(block1_ptr.borrow().get_pointer(Side::Right).is_some(), true);
    /// 
    /// // Set the left pointer of block1 to block2
    /// let prev_block_ptr = block1_ptr.borrow_mut().set_pointer(Some(&block2_ptr), Side::Left);
    /// assert_eq!(block1_ptr.borrow().get_pointer(Side::Left).is_some(), true);
    /// ```
    pub fn set_pointer(&mut self, new_block_ptr: Option<&Rc<RefCell<Block<T>>>>, side: Side) -> Option<Rc<RefCell<Block<T>>>> {
        match new_block_ptr {
            Some(new_block_ptr) => {
                match side {
                    Side::Left => {
                        self.left.replace(new_block_ptr.clone())
                    }
                    Side::Right => {
                        self.right.replace(new_block_ptr.clone())
                    }
                }
            },
            None => {
                match side {
                    Side::Left => {
                        self.left.take()
                    }
                    Side::Right => {
                        self.right.take()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_new() {
        let block_ptr = Block::new(10);
        assert_eq!(*block_ptr.borrow().read_data(), Some(10));
    }

    #[test]
    fn test_block_reference_count() {
        // Receive a new reference to the block, so there is two references to the block, this new pointer and the block inner pointer.
        let block_ptr = Block::new(10);
        assert_eq!(Rc::strong_count(&block_ptr), 1);

        {
             // Get a new references to the block
             // This will increase the reference count by 1
             #[allow(unused_variables)]
            let new_block_ptr_1 = block_ptr.borrow().get_reference();
            assert_eq!(Rc::strong_count(&block_ptr), 2);

            #[allow(unused_variables)]
            let new_block_ptr_2 = block_ptr.borrow().get_reference();
            assert_eq!(Rc::strong_count(&block_ptr), 3);
        }

        // The end of the prevous block should decrease the reference count by 1
        assert_eq!(Rc::strong_count(&block_ptr), 1);

        // Drop the last strong reference
        //drop(block_ptr);

        //assert_eq!(Rc::strong_count(&block_ptr), 0);
    }

    #[test]
    fn teste_block_set_rigth_pointer() {
        let block1_ptr = Block::new(10);
        let block2_ptr = Block::new(20);

        // Set the right pointer of block1 to block2
        let mut right_block_ptr = block1_ptr.borrow_mut().set_pointer(Some(&block2_ptr), Side::Right);
        assert_eq!(right_block_ptr.is_none(), true);
        
        // Read the data of the right block
        right_block_ptr = block1_ptr.borrow_mut().get_pointer(Side::Right);
        let binding = right_block_ptr.unwrap();
        let binding = binding.borrow();
        let right_block_data = binding.read_data();


        // In Rust it is necessary to divide the call as above to avoid temporary borrows issues with the compiler
        //let right_block_data = right_block_ptr.unwrap().borrow().read_data();

        assert_eq!(*right_block_data, Some(20));
    }

    #[test]
    fn test_block_get_data() {
        let block_ptr = Block::new(10);

        assert_eq!(block_ptr.borrow_mut().get_data(), Some(10));
        assert_eq!(block_ptr.borrow_mut().get_data(), None);
    }
}