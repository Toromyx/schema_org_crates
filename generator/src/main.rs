// https://github.com/mcarton/rust-derivative/issues/112
#![allow(clippy::non_canonical_partial_ord_impl)]

use indicatif::MultiProgress;
use read::read;
use write::write;

mod deprecated_attribute;
mod doc_lines;
mod feature;
mod read;
mod schema;
mod schema_org_crate;
mod schema_type;
mod sparql;
mod write;

#[tokio::main]
async fn main() {
	let multi_progress = MultiProgress::new();
	let store = read().await;
	write(&store, &multi_progress);
}
