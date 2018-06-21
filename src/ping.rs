use std::sync::mpsc::channel;
use std::thread;

pub fn ping(count: usize) -> usize {
    let (tx, rx) = channel();

    let t0 = thread::spawn(move || {
        for _ in 0..count {
            tx.send(()).unwrap();
        }
    });

    let t1 = thread::spawn(move || {
        let mut count = 0_usize;
        for _ in rx {
            count += 1;
        }
        count
    });

    t0.join().unwrap();
    t1.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(ping(1000), 1000);
    }
}
