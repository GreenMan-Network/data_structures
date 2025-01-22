//! This module defines a Vertex struct that represents a vertex in a linked list.
//! It includes methods for creating a new vertex, accessing and modifying the data, and managing pointers to the next and previous vertexes.
//! 
//! # Performance
//! - Accessing the data in a vertex is O(1).
//! - Updating the pointers to the next and previous vertex is O(1).
//! - Creating a new vertex is O(1).
//! 
//! # Usage
//! ```
//! ```
use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};

/// Direction of the pointer inside the Vertex
/// 
/// This enum is used to specify the direction of the pointer in a vertex of a doubly linked list.
/// It helps in identifying whether the pointer is pointing to the next vertex.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PointerName {
    Left,
    Right,
    Previous,
    Next,
    First,
    Last,
    To,
    From,
    Custom(String), // Custom pointer name for more flexibility
}

/// A Vertex in a linked list
/// # Fields
/// * `data`: The data contained in the vertex
/// * `self_ref`: A weak reference to the vertex itself
/// * `connections`: A HashMap that stores pointers to other vertexes in the list, allowing for bidirectional traversal.
/// 
#[derive(Debug)]
pub struct Vertex<T> {
    data: Option<T>,
    self_ref: Option<Weak<RefCell<Vertex<T>>>>,                      // reference to the vertex itself
    connections: HashMap<PointerName, Option<Rc<RefCell<Vertex<T>>>>>,  // vector of pointers to other vertexes
}

impl<T> Vertex<T> {
    /// Create a new vertex with the given data and return a pointer to it
    /// # Arguments
    /// * `num_pointers`: The number of pointers to other vertexes in the list
    /// 
    /// # Returns
    /// A pointer to the newly created vertex.
    /// A copy of the pointer is keep in the vertex itself, so if exists while the vertex is alive
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::vertex::Vertex;
    /// use std::rc::Rc;
    /// 
    /// let vertex_ptr = Vertex::new(10);
    /// ```
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        // Create new empty vertex
        let new_vertex_ptr = Rc::new(RefCell::new(Vertex {
            data: None,
            self_ref: None, // Temporariamente None
            connections: HashMap::new(),
        }));

        // Set the self_ref to point to itself
        new_vertex_ptr.borrow_mut().self_ref = Some(Rc::downgrade(&new_vertex_ptr));

        // Set the data in the new vertex
        new_vertex_ptr.borrow_mut().data = Some(data);

        new_vertex_ptr
    }

    /// Get a reference to the vertex itself keeping the reference count
    /// # Returns
    /// A reference to the vertex itself
    /// # Example
    /// ```
    /// use data_structures::linked_list::vertex::Vertex;
    /// use std::rc::Rc;
    /// 
    /// let vertex_ptr = Vertex::new(10);
    /// let new_vertex_ptr = vertex_ptr.borrow().get_reference();
    /// assert_eq!(Rc::strong_count(&vertex_ptr), 2);
    /// assert_eq!(Rc::strong_count(&new_vertex_ptr), 2);
    /// ```
    pub fn get_reference(&self) -> Rc<RefCell<Vertex<T>>> {
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
    /// use data_structures::linked_list::vertex::Vertex;
    /// let vertex_ptr = Vertex::new(10);
    /// assert_eq!(vertex_ptr.borrow().read_data().unwrap(), 10);
    /// ```
    /// 
    pub fn read_data(&self) -> &Option<T> {
        &self.data
    }

    /// Set the data of the vertex and return the old data
    /// # Arguments
    /// * `data`: The new data to be set in the vertex
    /// # Returns
    /// The old data of the vertex
    /// # Example
    /// ```
    /// use data_structures::linked_list::vertex::Vertex;
    /// 
    /// let mut vertex_ptr = Vertex::new(10);
    /// 
    /// assert_eq!(vertex_ptr.borrow_mut().set_data(20), Some(10));
    /// assert_eq!(vertex_ptr.borrow().read_data().unwrap(), 20);
    /// ```
    pub fn set_data(&mut self, data: T) -> Option<T> {
        self.data.replace(data)
    }

    /// Returns the data and erase all the pointers
    /// 
    /// # Returns
    /// The data contained in the vertex
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::vertex::Vertex;
    /// let vertex_ptr = Vertex::new(10);
    /// assert_eq!(vertex_ptr.borrow_mut().clear(), Some(10));
    /// ```
    /// 
    pub fn clear(&mut self) -> Option<T> {
        self.connections.clear();
        self.connections = HashMap::new(); // This was the only way I found to deallocate hasmap memory.

        self.self_ref.take();
        self.data.take()
    }

    /// Set a connection in the Vertex.
    /// If the connectio already exists, it will be replaced with the new one and return the old connection.
    ///
    /// # Arguments
    /// * `connection`: The new vertex to be set as the right pointer
    /// # Returns
    /// The old right vertex pointer
    /// # Example
    /// ```
    /// use data_structures::linked_list::vertex::Vertex;
    /// use data_structures::linked_list::vertex::PointerName;
    /// 
    /// let vertex1_ptr = Vertex::new(10);
    /// let vertex2_ptr = Vertex::new(20);
    /// 
    /// // Set the right pointer of vertex1 to vertex2
    /// let prev_vertex_ptr = vertex1_ptr.borrow_mut().set_connection(PointerName::Left, Some(&vertex2_ptr));
    /// 
    /// // Set the left pointer of vertex1 to vertex2
    /// let prev_vertex_ptr = vertex1_ptr.borrow_mut().set_connection(PointerName::Right, Some(&vertex2_ptr));
    /// ```
    pub fn set_connection(&mut self, pointer_name: PointerName, connection: Option<&Rc<RefCell<Vertex<T>>>>) -> Option<Rc<RefCell<Vertex<T>>>> {
        match connection {
            Some(new_connection) => {
                self.connections.insert(pointer_name, Some(new_connection.clone())).flatten()
            },
            None => {
                // If the pointer is None, remove it
                self.connections.insert(pointer_name, None).flatten()
            }
        }
    }

    /// This method returns a new copy of a pointer in the Vertex increasing the pointer counter.
    /// 
    /// # Returns
    /// A reference to the right pointer
    /// 
    /// # Example
    /// ```
    /// use data_structures::linked_list::vertex::Vertex;
    /// use data_structures::linked_list::vertex::PointerName;
    /// 
    /// let vertex_ptr = Vertex::new(10);
    /// let vertex_ptr2 = Vertex::new(20);
    /// 
    /// vertex_ptr.borrow_mut().set_connection(PointerName::Right, Some(&vertex_ptr2));
    /// 
    /// assert!(vertex_ptr.borrow().get_pointer(PointerName::Left).is_none());
    /// assert!(vertex_ptr.borrow().get_pointer(PointerName::Right).is_some());
    /// ```
    pub fn get_pointer(&self, pointer_name: PointerName) -> Option<Rc<RefCell<Vertex<T>>>> {
        match self.connections.get(&pointer_name) {
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
    fn test_vertex_new() {
        let vertex_ptr = Vertex::new(10);
        assert_eq!(*vertex_ptr.borrow().read_data(), Some(10));
    }

    #[test]
    fn test_vertex_reference_count() {
        // Receive a new reference to the vertex, so there is two references to the vertex, this new pointer and the vertex inner pointer.
        let vertex_ptr = Vertex::new(10);
        assert_eq!(Rc::strong_count(&vertex_ptr), 1);

        {
             // Get a new references to the vertex
             // This will increase the reference count by 1
             #[allow(unused_variables)]
            let new_vertex_ptr_1 = vertex_ptr.borrow().get_reference();
            assert_eq!(Rc::strong_count(&vertex_ptr), 2);

            #[allow(unused_variables)]
            let new_vertex_ptr_2 = vertex_ptr.borrow().get_reference();
            assert_eq!(Rc::strong_count(&vertex_ptr), 3);
        }

        // The end of the prevous vertex should decrease the reference count by 1
        assert_eq!(Rc::strong_count(&vertex_ptr), 1);

        // Drop the last strong reference
        //drop(vertex_ptr);

        //assert_eq!(Rc::strong_count(&vertex_ptr), 0);
    }

    #[test]
    fn teste_vertex_set_rigth_pointer() {
        let vertex1_ptr = Vertex::new(10);
        let vertex2_ptr = Vertex::new(20);

        // Set the right pointer vertex1 to vertex2
        let mut right_vertex_ptr = vertex1_ptr.borrow_mut().set_connection(PointerName::Right, Some(&vertex2_ptr));
        assert_eq!(right_vertex_ptr.is_none(), true);
        
        // Read the data of the right vertex
        right_vertex_ptr = vertex1_ptr.borrow_mut().get_pointer(PointerName::Right);
        let binding = right_vertex_ptr.unwrap();
        let binding = binding.borrow();
        let right_vertex_data = binding.read_data();


        // In Rust it is necessary to divide the call as above to avoid temporary borrows issues with the compiler
        //let right_vertex_data = right_vertex_data.unwrap().borrow().read_data();

        assert_eq!(*right_vertex_data, Some(20));
    }
}