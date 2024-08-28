mod outtermost {
    pub fn middle_function() {}

    pub fn middle_secret_funtion() {}

    pub mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {}
    }
}

fn try_me() {
    outtermost::middle_function();
    outtermost::middle_secret_funtion();

    outtermost::inside::inner_function();
    outtermost::inside::secret_function();
}