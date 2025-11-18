use crate::domain::models::path::Node;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(
	rename_all = "camelCase",
	rename_all_fields = "camelCase",
	tag = "event",
	content = "data"
)]
pub enum ScanEvent {
	Progress {
		junk_found: u128,
		current_path_str: String,
	},
	Finished {
		junk_found: u64,
		root_nodes: Vec<Node>,
	},
}


