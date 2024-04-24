#[derive(Debug, Clone)]
pub struct Stack<const SIZE: usize> {
    bytes: [u8; SIZE],
    stack_pointer: usize,
}

impl<const SIZE: usize> Stack<SIZE> {
    pub fn new() -> Self {
        Self {
            bytes: [0; SIZE],
            stack_pointer: 0,
        }
    }

    pub fn push<T>(&mut self, value: T) {
        if self.stack_pointer + core::mem::size_of::<T>() >= SIZE {
            panic!("VM stack push overflow");
        }

        unsafe {
            core::ptr::write(
                self.bytes.as_mut_ptr().add(self.stack_pointer) as *mut T,
                value,
            );
        }

        self.stack_pointer += core::mem::size_of::<T>();
    }

    pub fn pop<T>(&mut self) -> T {
        if self
            .stack_pointer
            .checked_sub(core::mem::size_of::<T>())
            .is_none()
        {
            panic!("VM stack pop underflow");
        }

        self.stack_pointer -= core::mem::size_of::<T>();

        unsafe { core::ptr::read(self.bytes.as_ptr().add(self.stack_pointer) as *const T) }
    }
}

impl<const SIZE: usize> Default for Stack<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn stack_test_u8() {
        let mut stack = Stack::<4096>::new();

        for _ in 0..2 {
            stack.push(1u8);
            stack.push(2u8);
            stack.push(3u8);
            stack.push(4u8);

            assert_eq!(stack.pop::<u8>(), 4);
            assert_eq!(stack.pop::<u8>(), 3);
            assert_eq!(stack.pop::<u8>(), 2);
            assert_eq!(stack.pop::<u8>(), 1);
        }
    }

    #[test]
    fn stack_test_u16() {
        let mut stack = Stack::<4096>::new();

        for _ in 0..2 {
            stack.push(50001u16);
            stack.push(50002u16);
            stack.push(50003u16);
            stack.push(50004u16);

            assert_eq!(stack.pop::<u16>(), 50004);
            assert_eq!(stack.pop::<u16>(), 50003);
            assert_eq!(stack.pop::<u16>(), 50002);
            assert_eq!(stack.pop::<u16>(), 50001);
        }
    }

    #[test]
    fn stack_test_u32() {
        let mut stack = Stack::<4096>::new();

        for _ in 0..2 {
            stack.push(50000001u32);
            stack.push(50000002u32);
            stack.push(50000003u32);
            stack.push(50000004u32);

            assert_eq!(stack.pop::<u32>(), 50000004);
            assert_eq!(stack.pop::<u32>(), 50000003);
            assert_eq!(stack.pop::<u32>(), 50000002);
            assert_eq!(stack.pop::<u32>(), 50000001);
        }
    }

    #[test]
    fn stack_test_u64() {
        let mut stack = Stack::<4096>::new();

        for _ in 0..2 {
            stack.push(438573948578934321u64);
            stack.push(438573948578934322u64);
            stack.push(438573948578934323u64);
            stack.push(438573948578934324u64);

            assert_eq!(stack.pop::<u64>(), 438573948578934324);
            assert_eq!(stack.pop::<u64>(), 438573948578934323);
            assert_eq!(stack.pop::<u64>(), 438573948578934322);
            assert_eq!(stack.pop::<u64>(), 438573948578934321);
        }
    }

    #[test]
    fn stack_test_varied() {
        let mut stack = Stack::<4096>::new();

        for _ in 0..2 {
            stack.push(438573948578934321u64);
            stack.push(50000002u32);
            stack.push(3u8);
            stack.push(50004u16);

            assert_eq!(stack.pop::<u16>(), 50004);
            assert_eq!(stack.pop::<u8>(), 3);
            assert_eq!(stack.pop::<u32>(), 50000002);
            assert_eq!(stack.pop::<u64>(), 438573948578934321);
        }
    }
}
