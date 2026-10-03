//! One node the HTTP API can list/get/replace/insert/delete. Mirrors Java's
//! `AddressableNode`, but carries a [`NodePath`] (indices) instead of a live
//! JavaParser-style object reference, since `syn` trees have no parent pointers
//! and Rust ownership forbids stashing mutable references across a request.
use crate::engine::node_path::NodePath;
#[derive(Debug, Clone)]
pub struct AddressableNode {
    pub id: String,
    pub path: NodePath,
    pub node_type: String,
    pub name: Option<String>,
    pub lineno: i64,
    pub end_lineno: i64,
    pub parent_type: String,
    pub expandable: bool,
    pub is_definition: bool,
}
