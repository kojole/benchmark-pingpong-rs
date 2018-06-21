use std::sync::mpsc::channel;
use std::thread;

pub fn pingpong(count: usize) -> usize {
    let (req_tx, req_rx) = channel();
    let (res_tx, res_rx) = channel();

    let t0 = thread::spawn(move || {
        for _ in 0..count {
            req_tx.send(()).unwrap();
            res_rx.recv().unwrap();
        }
    });

    let t1 = thread::spawn(move || {
        let mut count = 0_usize;
        for _ in req_rx {
            res_tx.send(()).unwrap();
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
        assert_eq!(pingpong(1000), 1000);
    }
}
