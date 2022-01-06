use std::{borrow::Borrow, cell::RefCell};
use std::sync::{Arc};
use tokio::sync::{Mutex,MutexGuard};


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
    pub inner: Arc<Mutex<Inner>>,
}

async fn test_main() {
    let inner = Inner {
        inner: String::new(),
    };

    let wrapper = Wrapper {
        inner: Arc::new(Mutex::new(inner)),
    };

    let inner = wrapper.inner.clone();
    tokio::spawn(async move {
        inner.lock().await.borrow().print().await;
    });

    println!("{}", &wrapper.inner.lock().await.borrow().inner);
}

