// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..100 {
            thread::sleep(Duration::from_millis(25));
            let mut sharedJobStatus: MutexGuard<JobStatus>  = status_shared.lock().unwrap();
            let curr_jobs_completed: u32 = sharedJobStatus.jobs_completed as u32;
            *sharedJobStatus = JobStatus { jobs_completed: curr_jobs_completed + 1 };
        }
    });
    while status.lock().unwrap().jobs_completed < 20 {
        println!("waiting... {}", status.lock().unwrap().jobs_completed);
        thread::sleep(Duration::from_millis(25));
    }
}
