use std::thread;

use crossbeam_channel::bounded;

pub fn cb_pingpong(count: usize) -> usize {
    let (req_tx, req_rx) = bounded(0);
    let (res_tx, res_rx) = bounded(0);

    let t0 = thread::spawn(move || {
        for _ in 0..count {
            req_tx.send(());
            res_rx.recv().unwrap();
        }
    });

    let t1 = thread::spawn(move || {
        let mut count = 0_usize;
        for _ in req_rx {
            res_tx.send(());
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
        assert_eq!(cb_pingpong(1000), 1000);
    }
}
