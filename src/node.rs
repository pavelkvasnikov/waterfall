pub mod node {
    pub struct Node {
        pub weight: f64,
        pub pushed: f64,
    }
    impl std::fmt::Display for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "W: {}, P: {}",
                   self.weight,
                   self.pushed
            )
        }
    }
}
