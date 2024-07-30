use prost::Message;

pub mod scip;

pub fn read_document(input: &[u8]) -> Option<scip::Document> {
    scip::Document::decode(input).ok()
}

pub fn read_index(input: &[u8]) -> Option<scip::Index> {
    scip::Index::decode(input).ok()
}
