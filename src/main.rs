use std::thread;
use std::sync::mpsc;

fn main() {
    let thread_count = 10;
    let mut handle_list = Vec::new();
    let mut threadtx_list = Vec::new();
    let (maintx, mainrx) = mpsc::channel();

    for _ in 0..thread_count {
        let maintx = maintx.clone();
        let (threadtx, threadrx) = mpsc::channel();
        threadtx_list.push(threadtx);
        let handle = thread::spawn(move || {
            'threadloop: loop {
                let number = threadrx.recv();
                let number: u64 = match number {
                    Ok(number) => number,
                    Err(_) => break 'threadloop,
                };
                for x in (2..).take_while(|n| n * n <= number) {
                    if number % x == 0 {
                        continue 'threadloop;
                    }
                }
                maintx.send(number).unwrap();
            }
        });
        handle_list.push(handle);
    }
    drop(maintx);

    let handle = thread::spawn(move || {
        let mut number: u64 = 2;
        'primeloop: loop {
            for threadtx in &threadtx_list {
                threadtx.send(number).unwrap();
                if number >= std::u64::MAX {
                    break 'primeloop;
                }
                number += 1;
            }
        }
    });
    handle_list.push(handle);

    loop {
        let prime = mainrx.recv();
        let prime = match prime {
            Ok(prime) => prime,
            Err(_) => break,
        };
        println!("{}", prime);
    }

    while let Some(handle) = handle_list.pop() {
        handle.join().unwrap();
    }
}
