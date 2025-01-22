use super::circular_queue::{CircularQueue, Direction};

pub struct FIFO<T> {
    fifo: CircularQueue<T>,
}

impl<T> FIFO<T> {
    /// Creates a new FIFO with a specified maximum size.
    /// If the maximum size is zero, the FIFO can grow indefinitely.
    /// # Arguments
    /// * `max_size` - The maximum number of elements the FIFO can hold.
    /// # Returns
    /// A new instance of FIFO.
    /// # Examples
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let fifo: FIFO<u32> = FIFO::new(5);
    ///
    /// assert_eq!(fifo.len(), 0);
    /// assert_eq!(fifo.max_size(), 5);
    /// ```
    pub fn new(max_size: usize) -> Self {
        FIFO {
            fifo: CircularQueue::new(max_size),
        }
    }

    /// Get the number of elements in the queue
    /// # Returns
    /// The number of elements in the queue
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let mut fifo: FIFO<u32> = FIFO::new(5);
    ///
    /// assert_eq!(fifo.len(), 0);
    ///
    /// fifo.push(1).unwrap();
    /// assert_eq!(fifo.len(), 1);
    ///
    /// fifo.push(2).unwrap();
    /// assert_eq!(fifo.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.fifo.len()
    }

    /// Check if the queue is empty
    /// # Returns
    /// True if the queue is empty, false otherwise
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let fifo: FIFO<u32> = FIFO::new(5);
    ///
    /// assert!(fifo.is_empty());
    /// ```
    ///
    pub fn is_empty(&self) -> bool {
        self.fifo.is_empty()
    }

    /// Check if the queue is full
    /// # Returns
    /// True if the queue is full, false otherwise
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let mut fifo: FIFO<u32> = FIFO::new(5);
    ///
    /// for i in 0..5 {
    ///    fifo.push(i).unwrap();
    /// }
    ///
    /// assert!(fifo.is_full());
    /// ```
    ///
    pub fn is_full(&self) -> bool {
        self.fifo.is_full()
    }

    /// Get the maximum size of the queue
    /// # Returns
    /// The maximum size of the queue
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let fifo: FIFO<u32> = FIFO::new(5);
    ///
    /// assert_eq!(fifo.max_size(), 5);
    /// ```
    pub fn max_size(&self) -> usize {
        self.fifo.max_size()
    }

    /// Set a new maximum size for the queue
    /// # Arguments
    /// * `max_size`: The new maximum size for the queue
    /// # Returns
    /// Result<(), &'static str>
    /// Ok if the new maximum size is set successfully, Err if the new maximum size is less than the current size
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let mut fifo: FIFO<i32> = FIFO::new(0);
    ///
    /// fifo.push(1);
    /// fifo.push(2);
    /// fifo.push(3);
    ///
    /// assert_eq!(fifo.set_max_size(2), Err("New max size is less than current size"));
    /// assert_eq!(fifo.set_max_size(3), Ok(()));
    ///
    /// assert_eq!(fifo.push(4), Err("Queue is full"));
    /// ```
    pub fn set_max_size(&mut self, max_size: usize) -> Result<(), &'static str> {
        self.fifo.set_max_size(max_size)
    }

    /// Push a new element to the begining of the queue
    /// # Arguments
    /// * `value` - The value to be added to the queue
    /// # Returns
    /// Result<(), &'static str>
    /// Ok(()) if the push was successful, Err("Queue is full") if the queue is full
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let mut fifo = FIFO::new(3);
    ///
    /// assert_eq!(fifo.push(1), Ok(()));
    /// assert_eq!(fifo.push(2), Ok(()));
    /// assert_eq!(fifo.push(3), Ok(()));
    /// assert_eq!(fifo.push(4), Err("Queue is full"));
    /// ```
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        self.fifo.insert(value, Direction::Left)
    }

    /// Pop an element from the end of the queue
    /// # Returns
    /// Option<T>
    /// Some(T) if the queue is not empty, None if the queue is empty
    /// # Example
    /// ```rust
    /// use data_structures::linked_list::fifo::FIFO;
    ///
    /// let mut fifo = FIFO::new(3);
    ///
    /// fifo.push(1).unwrap();
    /// fifo.push(2).unwrap();
    /// fifo.push(3).unwrap();
    /// assert_eq!(fifo.pop(), Some(1));
    /// assert_eq!(fifo.pop(), Some(2));
    /// assert_eq!(fifo.pop(), Some(3));
    /// assert_eq!(fifo.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.fifo.remove(Direction::Right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo() {
        let mut fifo = FIFO::new(3);

        assert_eq!(fifo.is_empty(), true);

        assert_eq!(fifo.push(1), Ok(()));
        assert_eq!(fifo.push(2), Ok(()));
        assert_eq!(fifo.push(3), Ok(()));

        assert_eq!(fifo.is_full(), true);

        assert_eq!(fifo.push(4), Err("Queue is full"));

        assert_eq!(fifo.pop(), Some(1));
        assert_eq!(fifo.pop(), Some(2));
        assert_eq!(fifo.pop(), Some(3));

        assert_eq!(fifo.pop(), None);
    }
}
