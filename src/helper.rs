use std::time::{SystemTime, UNIX_EPOCH};

pub fn wait_until_next_timestamp(current_timestamp: u64) -> u64 {
    let mut next_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    while next_timestamp <= current_timestamp {
        next_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }
    next_timestamp
}

pub fn get_valid_node_id(new_node_id: f64, node_id_bits: i32) -> i32 {
    let max_node_id = 1 << node_id_bits;
    let mut node_id: i32;

    if new_node_id.is_nan() {
        println!("Invalid node ID provided: {}, using default ID: 0", new_node_id);
        node_id = 0;
    } else {
        node_id = (new_node_id as i32) % max_node_id;
        if node_id < 0 {
            node_id = max_node_id - node_id.abs();
        }
    }

    node_id
}

pub mod defaults {
    pub const WORKER_ID: u32 = 0;
    pub const NODE_ID_BITS: i32 = 12;
    pub const SEQUENCE_BITS: i32 = 10;
    pub const EPOCH: u64 = 946684800000;
}