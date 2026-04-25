// Tim has to complete a few chores today, before he's allowed to play soccer
// with his friends. His friends decide to help him. Working together, they
// finish the chores earlier and have more time left to play soccer.
//
// Let's simulate this using asynchronous programming. Each boy is represented
// as an asynchronous task, which can be executed concurrently (they can be
// working at the same time).

use std::sync::atomic::{AtomicU8, Ordering};

// Used by "mom" to check that all chores are done before Tim plays soccer :-)
static CHORES_DONE: AtomicU8 = AtomicU8::new(0);

fn main() {
    // Async tasks need to be executed by a "runtime", which is not provided by
    // Rust's standard library. Here, we use the mainstream runtime `tokio`.
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    // TODO: Fix the compiler errors by making the spawned function async.
    let task_tim = rt.spawn(tim());
    let task_carl = rt.spawn(carl());
    let task_nick = rt.spawn(nick());

    // Block the runtime on a task that waits for all boys to finish the chores.
    // TODO: "await" all three tasks to fix the compiler errors.
    rt.block_on(async {
        task_tim;
        task_carl;
        task_nick;
    });

    assert_eq!(
        CHORES_DONE.load(Ordering::SeqCst),
        3,
        "Did you (a)wait for all the boys to finish the chores?"
    );
    println!("Ready to play soccer!");
}

fn tim() {
    println!("Cleaning my room...");
    CHORES_DONE.fetch_add(1, Ordering::SeqCst);
}

fn carl() {
    println!("Washing the dishes...");
    CHORES_DONE.fetch_add(1, Ordering::SeqCst);
}

fn nick() {
    println!("Mowing the lawn...");
    CHORES_DONE.fetch_add(1, Ordering::SeqCst);
}
