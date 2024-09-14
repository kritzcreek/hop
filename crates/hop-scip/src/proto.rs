use prost::Message;

pub mod scip;

pub fn read_document(input: &[u8]) -> Option<scip::Document> {
    scip::Document::decode(input).ok()
}

pub fn read_index(input: &[u8]) -> Option<scip::Index> {
    scip::Index::decode(input).ok()
}

pub fn encode_index(index: scip::Index) -> Vec<u8> {
    index.encode_to_vec()
}
