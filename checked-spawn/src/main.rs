

mod typed_thread {
    struct ThreadTokenInner {}
    pub struct ThreadToken(ThreadTokenInner);

    pub fn spawn<F, T>(f: F) -> std::thread::JoinHandle<T>
    where
        F: FnOnce(ThreadToken) -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let token = ThreadToken(ThreadTokenInner {});
        std::thread::spawn(|| f(token))
    }
}

use crate::typed_thread::{ThreadToken, spawn};

/// Blocking syscall wrapper
fn wait(t: &mut ThreadToken) {

    todo!()
}

/// Non-blocking code
fn poll(t: &ThreadToken) {
    todo!()
}

/// Allegedly non-blocking function
fn example_1(t: &ThreadToken) {

    // We can call non-blocking code, no problem
    poll(t);
    poll(t);
    poll(t);

    // We get type error when we try to call blocking function
    // wait(t);      <---- This doesn't type-check

    // We are forced to spawn in order to call the blocking function
    spawn(|mut token| {
        wait(&mut token)
    });
}

/// Self-reported blocking function
async fn example_2(t: &'static mut ThreadToken) {

    // We can call non-blocking code, no problem
    poll(t);
    poll(t);
    poll(t);

    // We can also wait
    wait(t);
    wait(t);
    wait(t);

    // We can poll in another task
    tokio::task::spawn(async {
        poll(&t)
    }).await;

    // But this will error
    // tokio::task::spawn(async move {
    //     poll(t)
    // });
}

fn main() {
    println!("Hello, world!");
}
