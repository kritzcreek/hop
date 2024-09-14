pub mod symbol;

#[allow(clippy::all)]
mod proto;

pub use proto::{encode_index, read_document, read_index, scip};
