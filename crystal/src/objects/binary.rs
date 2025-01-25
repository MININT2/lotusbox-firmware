use xxhash_rust::xxh3::xxh3_64;

pub enum MetadataFields {
    Name,
    Date,
    Time
}
pub struct Metadata {
    fields: Vec<MetadataFields>
}
pub struct RawData {
    data: Vec<u8>, // the raw binary data
    meta: Vec<Metadata>, // metadata map of regions
    hash: u64 // hash of the data inside
}
impl RawData {
    pub fn read(&self, index: u64) {

    }
    pub fn write(&mut self, data: Vec<u8>, meta: Vec<Metadata>) {
        self.data = data;
        self.hash = xxh3_64(self.data.as_slice());
        self.meta = meta;
    }
}
