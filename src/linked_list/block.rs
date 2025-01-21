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
//! //use data_structures::linked_list::block::{Block, Side};
//! 
//! //let block_ptr = Block::new(10);
//! //assert_eq!(*block_ptr.borrow().read_data(), Some(10));
//! ```
use std::{cell::RefCell, collections::HashMap, iter::Flatten, rc::{Rc, Weak}};

/// Direction of the pointer inside the Block
/// 
/// This enum is used to specify the direction of the pointer in a block of a doubly linked list.
/// It helps in identifying whether the pointer is pointing to the next block (Right) or the previous block (Left).
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PointerName {
    Left,
    Right,
    Previous,
    Next,
    First,
    Last,
    Custom(String), // Custom pointer name for more flexibility
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
    self_ref: Option<Weak<RefCell<Block<T>>>>,                      // reference to the block itself
    pointers: HashMap<PointerName, Option<Rc<RefCell<Block<T>>>>>,  // vector of pointers to other blocks
}

impl<T> Block<T> {
    /// Create a new block with the given data and return a pointer to it
    /// # Arguments
    /// * `num_pointers`: The number of pointers to other blocks in the list
    /// 
    /// # Returns
    /// A pointer to the newly created block.
    /// A copy of the pointer is keep in the block itself, so if exists while the block is alive
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// use std::rc::Rc;
    /// 
    /// let block_ptr = Block::new(10);
    /// ```
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        // Create new empty block
        let new_block_ptr = Rc::new(RefCell::new(Block {
            data: None,
            self_ref: None, // Temporariamente None
            pointers: HashMap::new(),
        }));

        // Set the self_ref to point to itself
        new_block_ptr.borrow_mut().self_ref = Some(Rc::downgrade(&new_block_ptr));

        // Set the data in the new block
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
        self.self_ref.as_ref().and_then(|weak_ref| weak_ref.upgrade()).unwrap()
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

    /// Returns the data and erase all the pointers
    /// 
    /// # Returns
    /// The data contained in the block
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// let block_ptr = Block::new(10);
    /// assert_eq!(block_ptr.borrow_mut().clear(), Some(10));
    /// ```
    /// 
    pub fn clear(&mut self) -> Option<T> {
        self.pointers.clear();
        self.pointers = HashMap::new(); // This was the only way I found to deallocate hasmap memory.

        self.self_ref.take();
        self.data.take()
    }

    /// Set a pointer in the Block.
    /// If the point already exists, it will be replaced with the new one and return the old pointer.
    ///
    /// # Arguments
    /// * `new_block_ptr`: The new block to be set as the right pointer
    /// # Returns
    /// The old right block pointer
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// use data_structures::linked_list::block::PointerName;
    /// 
    /// let block1_ptr = Block::new(10);
    /// let block2_ptr = Block::new(20);
    /// 
    /// // Set the right pointer of block1 to block2
    /// let prev_block_ptr = block1_ptr.borrow_mut().set_pointer(PointerName::Left, Some(&block2_ptr));
    /// 
    /// // Set the left pointer of block1 to block2
    /// let prev_block_ptr = block1_ptr.borrow_mut().set_pointer(PointerName::Right, Some(&block2_ptr));
    /// ```
    pub fn set_pointer(&mut self, pointer_name: PointerName, new_block_ptr: Option<&Rc<RefCell<Block<T>>>>) -> Option<Rc<RefCell<Block<T>>>> {
        match new_block_ptr {
            Some(new_ptr) => {
                self.pointers.insert(pointer_name, Some(new_ptr.clone())).flatten()
            },
            None => {
                // If the pointer is None, remove it
                self.pointers.insert(pointer_name, None).flatten()
            }
        }
    }

    /// This method returns a new copy of a pointer in the Block increasing the pointer counter.
    /// 
    /// # Returns
    /// A reference to the right pointer
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::block::Block;
    /// use data_structures::linked_list::block::PointerName;
    /// 
    /// let block_ptr = Block::new(10);
    /// let block_ptr2 = Block::new(20);
    /// 
    /// block_ptr.borrow_mut().set_pointer(PointerName::Right, Some(&block_ptr2));
    /// 
    /// assert!(block_ptr.borrow().get_pointer(PointerName::Left).is_none());
    /// assert!(block_ptr.borrow().get_pointer(PointerName::Right).is_some());
    /// ```
    pub fn get_pointer(&self, pointer_name: PointerName) -> Option<Rc<RefCell<Block<T>>>> {
        match self.pointers.get(&pointer_name) {
            Some(ptr) => {
                ptr.clone()
            }
            None => None    // In this case there is no key with pointer_name.
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
        let mut right_block_ptr = block1_ptr.borrow_mut().set_pointer(PointerName::Right, Some(&block2_ptr));
        assert_eq!(right_block_ptr.is_none(), true);
        
        // Read the data of the right block
        right_block_ptr = block1_ptr.borrow_mut().get_pointer(PointerName::Right);
        let binding = right_block_ptr.unwrap();
        let binding = binding.borrow();
        let right_block_data = binding.read_data();


        // In Rust it is necessary to divide the call as above to avoid temporary borrows issues with the compiler
        //let right_block_data = right_block_ptr.unwrap().borrow().read_data();

        assert_eq!(*right_block_data, Some(20));
    }
}