use some_module::SomeType;

impl Bla {
    fn test(&self, error: &SomeError) -> bool {
        if let Some(val) = error.downcast_ref::<SomeType>() {
            return true;
        }
        false
    }
}