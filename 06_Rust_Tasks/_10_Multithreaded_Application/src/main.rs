use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Local}; 

#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    record_added_time: SystemTime,
    thread_id: String,
}

fn main() {
    println!("🚀 Multithreading application started...");

    // Shared data and global counter
    let records = Arc::new(Mutex::new(Vec::<MultiThread>::new()));
    let counter = Arc::new(AtomicI32::new(1));

    // ---------------- THREAD 1 (Record Creator) ----------------
    let r1 = Arc::clone(&records);
    let c1 = Arc::clone(&counter);
    thread::spawn(move || {
        println!("✅ Thread 1 started (Creator)");
        loop {
            let id = c1.fetch_add(1, Ordering::SeqCst);
            let record = MultiThread {
                id,
                record_added_time: SystemTime::now(),
                thread_id: format!("T-{}", rand::random::<u32>()),
            };
            {
                let mut data = r1.lock().unwrap();
                data.push(record);
            }
            println!("[T1] Added record with id {}", id);
            thread::sleep(Duration::from_secs(10));
        }
    });

    // ---------------- THREAD 2 (State Printer) ----------------
    let r2 = Arc::clone(&records);
    thread::spawn(move || {
        println!("✅ Thread 2 started (Printer)");
        loop {
            thread::sleep(Duration::from_secs(5));
            {
                let data = r2.lock().unwrap();
                println!("\n--- Current State ---");
                for rec in data.iter() {
                    let datetime: DateTime<Local> = rec.record_added_time.into();
                    let human_time = datetime.format("%I:%M:%S %p").to_string().to_lowercase();
                    
                    println!("[ID: {} | Time: {} | UID: {}]", 
                        rec.id, human_time, rec.thread_id);
                }
                println!("----------------------");
            }
        }
    });

    // ---------------- THREAD 3 (Even Record Cleaner) ----------------
    let r3 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 3 started (Even Cleaner)");
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut data = r3.lock().unwrap();
            let now = SystemTime::now();

            data.retain(|rec| {
                let is_even = rec.id % 2 == 0;
                let age = now.duration_since(rec.record_added_time)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();

                if is_even && age > 20 {
                    println!("🧹 [T3] Removing Even ID: {}", rec.id);
                    false 
                } else {
                    true 
                }
            });
        }
    });

    // ---------------- THREAD 4 (Odd Record Cleaner) ----------------
    let r4 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 4 started (Odd Cleaner)");
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut data = r4.lock().unwrap();
            let now = SystemTime::now();

            data.retain(|rec| {
                let is_odd = rec.id % 2 != 0;
                let age = now.duration_since(rec.record_added_time)
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();

                if is_odd && age > 20 {
                    println!("[T4] Removing Odd ID: {}", rec.id);
                    false 
                } else {
                    true 
                }
            });
        }
    });

    // ---------------- THREAD 5 (Even Counter) ----------------
    let r5 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 5 started (Even Counter)");
        loop {
            thread::sleep(Duration::from_secs(4));
            let data = r5.lock().unwrap();
            let count = data.iter().filter(|r| r.id % 2 == 0).count();
            println!("[T5] Total Even records: {}", count);
        }
    });

    // ---------------- THREAD 6 (Odd Counter) ----------------
    let r6 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 6 started (Odd Counter)");
        loop {
            thread::sleep(Duration::from_secs(4));
            let data = r6.lock().unwrap();
            let count = data.iter().filter(|r| r.id % 2 != 0).count();
            println!("[T6] Total Odd records: {}", count);
        }
    });

    // Main thread wait loop
    loop { 
        thread::sleep(Duration::from_secs(60)); 
    }
}