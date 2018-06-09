mod outermost {
    pub fn middle_func() {

    }

    pub fn middle_secret_func() {}

    pub mod inside {
        pub fn inner_func() {
            ::outermost::middle_secret_func();
        }
        fn secret_func() {}

    }
}

fn try_me() {
    outermost::middle_func();
    outermost::middle_secret_func();
    outermost::inside::inner_func();
//    outermost::inside::secret_func();
}
