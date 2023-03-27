pub trait Create {
    fn get(&self, title: &str) {
        println("'' is being created.", title);
    }
}
