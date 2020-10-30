use std::sync::{Arc, Mutex};

struct Counter {
    inner: Arc<Mutex<usize>>,
}

impl Counter {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(0)),
        }
    }

    fn incr(&self) -> usize {
        let mut w = self.inner.lock().unwrap();
        *w += 1;
        *w
    }
}

fn main() {
    let counter = Counter::new();
    counter.incr();
    println!("Counter value is: {}", counter.incr());

    println!("Max of usize: {}", usize::MAX);
    println!("Hello, world!");
}
