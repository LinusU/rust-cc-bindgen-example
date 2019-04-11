mod bindings;

pub struct Adder {
    c_handle: bindings::Adder,
}

impl Adder {
    pub fn new() -> Adder {
        Adder { c_handle: unsafe { bindings::Adder::new() } }
    }

    pub fn set_left(&mut self, left: i32) {
        unsafe { self.c_handle.setLeft(left); }
    }

    pub fn set_right(&mut self, right: i32) {
        unsafe { self.c_handle.setRight(right); }
    }

    pub fn calculate(&mut self) -> i32 {
        unsafe { self.c_handle.calculate() }
    }
}

impl Drop for Adder {
    fn drop(&mut self) {
        unsafe { bindings::Adder_Adder_destructor(&mut self.c_handle); }
    }
}

#[cfg(test)]
mod tests {
    use super::Adder;

    #[test]
    fn it_works() {
        let mut adder = Adder::new();

        adder.set_left(1);
        adder.set_right(2);

        assert_eq!(adder.calculate(), 3);
    }
}
