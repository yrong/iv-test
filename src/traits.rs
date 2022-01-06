// TODO: modify this file to pass compile and keep function.
// 调整代码使得此文件可以把正常编译，保持功能不发生变化。
//
// 尽量少的修改代码。


pub trait TestTrait {
    type Error;
}

impl TestTrait for () {
    type Error = ();
}

pub struct Test<'a> {
    inner: Vec<&'a dyn TestTrait<Error = ()>>,
}

impl<'a> Test<'a> {
    pub fn add(&mut self, v: &'a impl TestTrait<Error = ()>) {
        self.inner.push(v);
    }
}

