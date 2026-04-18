// Two people are talking on the phone. One of them is telling a story. The
// other one is interjecting with little acknowledgments, to show their interest
// in the story.
//
// However, there is a problem. The phone connection is synchronous, so all
// the acknowledgments from the listener arrive only at the very end of the
// conversation! What the speaker and listener say should be interleaved.
//
// Let's use asynchronous programming to make the conversation more natural!

use std::time::Duration;

use tokio::sync::mpsc;

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    let _guard = rt.enter();

    let time_scale = Duration::from_millis(1);

    let (speaker_phone, listener_phone, mut wire_tap) = start_wire_tapped_phone_call();

    let speaker = async move {
        for msg in SPEAKER_MESSAGES {
            speaker_phone.say(msg).await;
            // wait for listener to interject
            wait_silently(time_scale * 2).await;
        }
    };
    let listener = async move {
        // give speaker a head-start
        wait_silently(time_scale * 1).await;
        for msg in LISTENER_MESSAGES {
            listener_phone.say(msg).await;
            // wait for speaker to continue story
            wait_silently(time_scale * 2).await;
        }
    };
    tokio::spawn(speaker);
    tokio::spawn(listener);

    let messages: Vec<_> = std::iter::from_fn(|| rt.block_on(wire_tap.recv())).collect();
    for message in &messages {
        println!("{message}");
    }
    let expected = SPEAKER_MESSAGES
        .iter()
        .zip(LISTENER_MESSAGES)
        .flat_map(|(&a, &b)| [a, b]);
    for (expected, message) in expected.zip(messages) {
        assert_eq!(message, expected, "")
    }
}

async fn wait_silently(duration: Duration) {
    // TODO: The sleep function from the standard library blocks the current
    // thread, preventing other async tasks from progressing. The tokio
    // library, which provides our async runtime, can help:
    // https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
    std::thread::sleep(duration);
}

const SPEAKER_MESSAGES: &[&str] = &[
    "> So I was walking in the park...",
    "> where I met Susan by coincidence...",
    "> and she was wearing a purple hat!",
];
const LISTENER_MESSAGES: &[&str] = &[
    "                                          I see. <",
    "                                     Oh, really? <",
    "                                         No way! <",
];

/// This phone is wire-tapped for testing purposes.
#[derive(Clone)]
struct Phone {
    sender: mpsc::Sender<&'static str>,
}

// Create a wire-tapped phone call.
fn start_wire_tapped_phone_call() -> (Phone, Phone, mpsc::Receiver<&'static str>) {
    let (sender, wire_tap) = mpsc::channel(6);
    let phone = Phone { sender };
    (phone.clone(), phone, wire_tap)
}

impl Phone {
    /// Say something on the phone.
    async fn say(&self, thing: &'static str) {
        self.sender.send(thing).await.unwrap();
    }
}
