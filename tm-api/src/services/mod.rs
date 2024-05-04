pub mod process;

// Return this if condition is true, return that if false
pub trait ThisOrThat<Out> {
    fn this_or_that(self, condition: bool) -> Out;
}

impl<T> ThisOrThat<T> for (T, T) {
    fn this_or_that(self, condition: bool) -> T {
        if condition {
            self.0
        } else {
            self.1
        }
    }
}
