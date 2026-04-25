// Alice is an elementary school teacher who needs to calculate the mean test
// score for three classes she teaches. Instead of calculating them one after
// the other, she decides to ask her friends Bob and Catherine for help. Working
// together, they can finish the job much faster.
//
// Let's simulate this using asynchronous programming. Each person is
// represented as an asynchronous task, which can be executed concurrently (i.e.
// they can be doing the calculations at the same time).

fn main() {
    // Async tasks need to be executed by a "runtime", which is not provided by
    // Rust's standard library. Here, we use the mainstream runtime `tokio`.
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    let scores_class_a = &[83, 77, 92];
    let scores_class_b = &[84, 88, 96];
    let scores_class_c = &[71, 83, 76];

    let alice = rt.spawn(calculate_mean_score(scores_class_a));
    let bob = rt.spawn(calculate_mean_score(scores_class_b));
    let catherine = rt.spawn(calculate_mean_score(scores_class_c));

    // Block the runtime on a task that awaits all three calculations.
    let [mean_score_a, mean_score_b, mean_score_c]: [usize; _] = rt.block_on(async {
        [
            alice.await.unwrap(),
            bob.await.unwrap(),
            catherine.await.unwrap(),
        ]
    });

    assert_eq!(mean_score_a, 84);
    assert_eq!(mean_score_b, 89);
    assert_eq!(mean_score_c, 76);
}

async fn calculate_mean_score(score_list: &[usize]) -> usize {
    let score_sum: usize = score_list.iter().sum();
    score_sum / score_list.len()
}
