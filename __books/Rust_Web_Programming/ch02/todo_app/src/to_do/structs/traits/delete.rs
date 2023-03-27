pub trait Delete {
    fn get(&self, title: &str) {
        println!("'{}' is being deleted.", title);
    }
}
