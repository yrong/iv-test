pub struct IterTest {
    pub inner: Vec<String>,
}

impl IterTest {
    fn find_and_update(&mut self) {
        // TODO: modify to pass compile.
        for mut v in &self.inner {
            if v == "" {
                v = &"test".to_string()
            }
        }
    }
}

