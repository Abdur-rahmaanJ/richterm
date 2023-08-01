use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

struct Uuid(u128);

impl Uuid {
    fn new() -> Self {
        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let uuid_value =
            u128::from(timestamp.as_secs()) << 64 | u128::from(timestamp.subsec_nanos());
        Uuid(uuid_value)
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032X}", self.0)
    }
}

pub fn generate_random_uuid() -> String {
    let uuid = Uuid::new();

    format!("{}", uuid)
}
