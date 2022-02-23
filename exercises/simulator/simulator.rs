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
    Server(IPAddrKind),
    Switch(MacAddrKind),
    Router(MacAddrKind),
}

struct Device {
    dtype: DeviceType,
    name: String,
}

fn main() {
    // create a hashmap of pairs <Device, Vec[Device]>
    // "Desk1" [Desktop, V4] get connected to "Switch2", "Router1" [Router, Virtual]
    // "Desk2" [Desktop, V6] get connected to "Switch1", "Router1"
    // "Desk3" [Desktop, V4] get connected to "Server1" [Server, V4]
    // "Switch1" [Switch, PHYSICAL] get connected to "Router1"
    // "Switch2" [Switch, PHYSICAL] get connected to "Router2" [Router, Virtual]
    // "Server2" [Server, V6] get connected to "Router2"
    // Note: connections are suppose to be bidirectional, so write your methods in a way so that they would automatically handled

    // Given a device name, return who are connected to it

    // Given an IPAddrKind, return devices ip addresses who uses specific IPAddrKind

    // Given Device1 and Device2, return true, if they are connected (including through other nodes)

    // Given a device name, remove it from all of its connection
}
