use xxhash_rust::xxh3::xxh3_64;

pub struct RawData {
    data: Vec<u8>, // the raw binary data
    meta: Vec<u64>, // metadata map of regions
    hash: Vec<u64> // hash of the data inside
}
impl RawData {
    pub fn read(&self, index: u64) {

    }
    pub fn write(&self, data: Vec<u8>, )
}
