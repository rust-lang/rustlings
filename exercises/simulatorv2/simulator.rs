use std::vec;

enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum MacAddrKind {
    PHYSICAL(u64),
    VIRTUAL(u32),
}

enum DeviceType {
    Desktop(IPAddrKind),
    Switch(MacAddrKind),
    Router(MacAddrKind),
}

struct Device {
    dtype: DeviceType,
    name: String,
}

fn dir_conn_to(query: String) -> Vec<String> {
    let result = Vec::new();

    result
}

fn find_ip_kind(query: IPAddrKind) -> Vec<String> {
    let result = Vec::new();

    result
}

fn can_talk(src: String, dst: String) -> Vec<String> {
    let result = Vec::new();

    result
}

fn rm_device(device: String) -> bool {
    true
}

fn main() {
    // you can maintain a hashmap of <device_name: String, device: Device> alongside of the other hashmap of <device: Device, list: vector<Device>>
    // Note, you are free to choose any type including Box<T>, Rc<T>, RefCell<T>, etc.

    // inputs will be taken from standard i/o. A file has attached to sample.

    assert_eq!(
        vec![
            String::from("Switch1"),
            String::from("Switch2"),
            String::from("Desk2")
        ],
        dir_conn_to(String::from("Router1"))
    );

    assert_eq!(
        vec![String::from("Switch1"), String::from("Switch2")],
        find_ip_kind(IPAddrKind::V6(String::from("")))
    );

    assert_eq!(
        vec![
            String::from("Desk1"),
            String::from("Switch1"),
            String::from("Router1"),
            String::from("Switch2"),
            String::from("Desk3")
        ],
        can_talk(String::from("Desk1"), String::from("Desk3"))
    );

    assert!(rm_device(String::from("Switch2")));

    let exp_vec: Vec<String> = Vec::new();
    assert_eq!(
        exp_vec,
        can_talk(String::from("Desk1"), String::from("Desk3"))
    );
}
