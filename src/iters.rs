pub struct IterTest {
    pub inner: Vec<String>,
}

impl IterTest {
    fn find_and_update(&mut self) {
        // TODO: modify to pass compile.
        for v in self.inner {
            if v == "" {
                v = "test"
            }
        }
    }
}

