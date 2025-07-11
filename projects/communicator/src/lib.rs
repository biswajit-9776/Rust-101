mod outermost {
    pub fn middle_function() {
        print!("iefij");
    }
    fn middle_secret_function() {
        print!("ejfie");
    }

    pub mod inside {
        use crate::outermost::{middle_function, middle_secret_function};

        pub fn inner_function() {
            print!("rgr");
        }
        fn secret() {
            print!("oekfok");
            middle_secret_function();
            middle_function();
        }

    }
}
fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();
    outermost::inside::inner_function();
    // outermost::inside::secret();
}
mod another_outermost{
    use crate::outermost;
    fn try_me() {
        outermost::middle_function();
    }
}