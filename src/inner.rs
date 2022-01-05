use std::{borrow::Borrow, cell::RefCell};

// TODO: modify this file to pass compile and keep function.
// 调整代码使得此文件可以把正常编译，保持功能不发生变化。
//
// 尽量少的修改代码。

pub struct Inner {
    pub inner: String,
}

impl Inner {
    // Important! Don't change this function define.
    pub async fn print(&self) {
        println!("{}", self.inner);
    }
}

pub struct Wrapper {
    pub inner: RefCell<Inner>,
}

pub fn test_main() {
    let inner = Inner {
        inner: String::new(),
    };

    let wrapper = Wrapper {
        inner: RefCell::new(inner),
    };

    tokio::spawn(async {
        wrapper.inner.borrow().print().await;
    });

    println!("{}", &wrapper.inner.borrow().inner);
}

