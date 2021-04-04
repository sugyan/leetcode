pub struct MyCircularQueue {
    v: Vec<i32>,
    front: usize,
    rear: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            v: vec![0; k as usize + 1],
            front: 0,
            rear: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.v[self.rear] = value;
        self.rear = (self.rear + 1) % self.v.len();
        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.front = (self.front + 1) % self.v.len();
        true
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[self.front]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.v[(self.rear + self.v.len() - 1) % self.v.len()]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    pub fn is_full(&self) -> bool {
        (self.rear + 1) % self.v.len() == self.front
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = MyCircularQueue::new(3);
        assert_eq!(true, obj.en_queue(1));
        assert_eq!(true, obj.en_queue(2));
        assert_eq!(true, obj.en_queue(3));
        assert_eq!(false, obj.en_queue(4));
        assert_eq!(3, obj.rear());
        assert_eq!(true, obj.is_full());
        assert_eq!(true, obj.de_queue());
        assert_eq!(true, obj.en_queue(4));
        assert_eq!(4, obj.rear());
    }
}
