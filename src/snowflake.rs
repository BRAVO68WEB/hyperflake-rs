use crate::helper::{get_valid_node_id, wait_until_next_timestamp, defaults};

pub struct SnowflakeId {
    node_id_bits: i32,
    sequence_bits: i32,
    epoch: u64,
    last_timestamp: i64,
    sequence: u32,
    node_id: i32,
    max_sequence: u32,
}

impl SnowflakeId {
    pub fn new() -> Self {
        let worker_id = defaults::WORKER_ID;
        let node_id_bits = defaults::NODE_ID_BITS;
        let sequence_bits = defaults::SEQUENCE_BITS;
        let epoch = defaults::EPOCH;

        let last_timestamp = -1;
        let sequence = 0;
        let node_id = get_valid_node_id(worker_id as f64, node_id_bits);
        let max_sequence = (1 << sequence_bits) - 1;

        SnowflakeId {
            node_id_bits,
            sequence_bits,
            epoch,
            last_timestamp,
            sequence,
            node_id,
            max_sequence,
        }
    }

    pub fn generate(&mut self) -> String {
        let timestamp: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        if timestamp < self.last_timestamp {
            panic!("Clock is moving backwards!");
        }

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & self.max_sequence;
            if self.sequence == 0 {
                let new_timestamp = wait_until_next_timestamp(timestamp as u64);
                self.last_timestamp = new_timestamp as i64;
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        let high = ((timestamp - self.epoch as i64) << (self.node_id_bits + self.sequence_bits))
            | ((self.node_id as i64) << self.sequence_bits)
            | self.sequence as i64;

        let snowflake_id = high;

        snowflake_id.to_string()
    }

    pub fn decode(&self, hfid: &str) -> u64 {
        let high = hfid.parse::<i64>().unwrap();
        let timestamp = ((high >> (self.node_id_bits + self.sequence_bits)) + self.epoch as i64) as u64;
        timestamp
    }
}