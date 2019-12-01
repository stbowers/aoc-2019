#[cfg(feature = "text_utils")]
pub use crate::text_utils::*;

#[cfg(feature = "file_utils")]
pub use crate::file_utils::*;

pub use std::collections::{
    BTreeSet as Set, BTreeMap as Map, VecDeque, LinkedList, HashMap, HashSet, BinaryHeap
};
