use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::super::l020::a::model_descriptor::{ModelDescriptor, Normalization, Pooling};
use super::super::l020::l010::layout::DType;

pub type EntryId = u64;
pub type Stamp = u64;

const MAGIC: &[u8; 8] = b"XYRGIDX\0";
const FORMAT_VERSION: u32 = 1;
const MIN_LOG_CHANGES: usize = 10_000;
const MIN_ENTRY_BYTES: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub model_id: String,
    pub dimension: usize,
    pub storage_dtype: DType,
    pub pooling: Pooling,
    pub normalization: Normalization,
    pub producing_profile: Option<String>,
}

fn dtype_size(d: DType) -> usize {
    match d {
        DType::F32 | DType::U32 => 4,
        DType::F16 | DType::BF16 => 2,
        DType::U8 => 1,
        DType::I64 => 8,
    }
}

impl IndexHeader {
    pub fn from_descriptor(d: &ModelDescriptor, producing_profile: Option<String>) -> IndexHeader {
        IndexHeader {
            model_id: d.model_id.clone(),
            dimension: d.dimension,
            storage_dtype: d.storage_dtype,
            pooling: d.pooling,
            normalization: d.normalization,
            producing_profile,
        }
    }

    pub fn vector_bytes(&self) -> usize {
        self.dimension.saturating_mul(dtype_size(self.storage_dtype))
    }

    pub fn is_compatible(&self, other: &IndexHeader) -> bool {
        self.model_id == other.model_id
            && self.dimension == other.dimension
            && self.storage_dtype == other.storage_dtype
            && self.pooling == other.pooling
            && self.normalization == other.normalization
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: EntryId,
    pub parent: Option<EntryId>,
    pub payload: Vec<u8>,
    pub vectors: Vec<u8>,
    pub stamp: Stamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentRef {
    Root,
    Entry(EntryId),
    New(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Insert {
        parent: ParentRef,
        payload: Vec<u8>,
        vectors: Vec<u8>,
    },
    Update {
        id: EntryId,
        payload: Option<Vec<u8>>,
        vectors: Option<Vec<u8>>,
    },
    Move {
        id: EntryId,
        parent: Option<EntryId>,
        position: Option<usize>,
    },
    Remove {
        id: EntryId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    Inserted,
    Updated,
    Moved,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Change {
    pub stamp: Stamp,
    pub id: EntryId,
    pub kind: ChangeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub from: Stamp,
    pub to: Stamp,
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Applied {
    pub stamp: Stamp,
    pub inserted: Vec<EntryId>,
}

#[derive(Debug)]
pub enum IndexError {
    ModelMismatch {
        expected: IndexHeader,
        found: IndexHeader,
    },
    Corrupt(String),
    Io(io::Error),
    NotFound(EntryId),
    InvalidVector {
        expected_multiple_of: usize,
        got: usize,
    },
    InvalidParent(String),
    HistoryTruncated {
        oldest: Stamp,
    },
}

impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexError::ModelMismatch { expected, found } => write!(
                f,
                "index model mismatch: expected {}/{}, found {}/{}",
                expected.model_id, expected.dimension, found.model_id, found.dimension
            ),
            IndexError::Corrupt(m) => write!(f, "corrupt index: {m}"),
            IndexError::Io(e) => write!(f, "index i/o error: {e}"),
            IndexError::NotFound(id) => write!(f, "entry {id} not found"),
            IndexError::InvalidVector {
                expected_multiple_of,
                got,
            } => write!(
                f,
                "invalid vector length {got}, expected a multiple of {expected_multiple_of}"
            ),
            IndexError::InvalidParent(m) => write!(f, "invalid parent: {m}"),
            IndexError::HistoryTruncated { oldest } => {
                write!(f, "change history truncated, oldest stamp {oldest}")
            }
        }
    }
}

impl std::error::Error for IndexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IndexError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for IndexError {
    fn from(e: io::Error) -> Self {
        IndexError::Io(e)
    }
}

fn corrupt(msg: impl Into<String>) -> IndexError {
    IndexError::Corrupt(msg.into())
}

#[derive(Debug, Clone)]
struct Node {
    parent: Option<EntryId>,
    children: Vec<EntryId>,
    payload: Vec<u8>,
    vectors: Vec<u8>,
    stamp: Stamp,
}

enum Undo {
    Insert {
        id: EntryId,
        parent: Option<EntryId>,
    },
    Update {
        id: EntryId,
        payload: Option<Vec<u8>>,
        vectors: Option<Vec<u8>>,
    },
    Move {
        id: EntryId,
        old_parent: Option<EntryId>,
        old_index: usize,
        new_parent: Option<EntryId>,
    },
    Remove {
        id: EntryId,
        parent: Option<EntryId>,
        index: usize,
        nodes: Vec<(EntryId, Node)>,
    },
}

#[derive(Default)]
struct Batch {
    undo: Vec<Undo>,
    changes: Vec<Change>,
    touched: Vec<EntryId>,
    inserted_by_op: Vec<Option<EntryId>>,
    inserted: Vec<EntryId>,
    structural: bool,
}

struct State {
    header: IndexHeader,
    nodes: BTreeMap<EntryId, Node>,
    roots: Vec<EntryId>,
    next_id: EntryId,
    stamp: Stamp,
    structure_stamp: Stamp,
    log: VecDeque<Change>,
    oldest: Stamp,
}

impl State {
    fn empty(header: IndexHeader) -> State {
        State {
            header,
            nodes: BTreeMap::new(),
            roots: Vec::new(),
            next_id: 1,
            stamp: 0,
            structure_stamp: 0,
            log: VecDeque::new(),
            oldest: 0,
        }
    }

    fn siblings(&self, parent: Option<EntryId>) -> &[EntryId] {
        match parent {
            None => &self.roots,
            Some(p) => self
                .nodes
                .get(&p)
                .map(|n| n.children.as_slice())
                .unwrap_or(&[]),
        }
    }

    fn siblings_mut(&mut self, parent: Option<EntryId>) -> Option<&mut Vec<EntryId>> {
        match parent {
            None => Some(&mut self.roots),
            Some(p) => self.nodes.get_mut(&p).map(|n| &mut n.children),
        }
    }

    fn detach(&mut self, parent: Option<EntryId>, id: EntryId) -> Option<usize> {
        let list = self.siblings_mut(parent)?;
        let idx = list.iter().position(|x| *x == id)?;
        list.remove(idx);
        Some(idx)
    }

    fn attach(&mut self, parent: Option<EntryId>, id: EntryId, index: usize) -> Option<usize> {
        let list = self.siblings_mut(parent)?;
        let idx = index.min(list.len());
        list.insert(idx, id);
        Some(idx)
    }

    fn preorder_from(&self, root: EntryId) -> Vec<EntryId> {
        let mut out = Vec::new();
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            out.push(id);
            if let Some(n) = self.nodes.get(&id) {
                stack.extend(n.children.iter().rev().copied());
            }
        }
        out
    }

    fn check_vectors(&self, v: &[u8]) -> Result<(), IndexError> {
        let vb = self.header.vector_bytes();
        let ok = if vb == 0 { v.is_empty() } else { v.len() % vb == 0 };
        if ok {
            Ok(())
        } else {
            Err(IndexError::InvalidVector {
                expected_multiple_of: vb,
                got: v.len(),
            })
        }
    }

    fn apply_ops(&mut self, ops: Vec<Op>) -> Result<(Applied, Vec<Change>), IndexError> {
        if ops.is_empty() {
            return Ok((
                Applied {
                    stamp: self.stamp,
                    inserted: Vec::new(),
                },
                Vec::new(),
            ));
        }
        let new_stamp = self.stamp + 1;
        let saved_next = self.next_id;
        let mut batch = Batch {
            inserted_by_op: vec![None; ops.len()],
            ..Batch::default()
        };
        if let Err(e) = self.run_ops(ops, new_stamp, &mut batch) {
            self.rollback(std::mem::take(&mut batch.undo));
            self.next_id = saved_next;
            return Err(e);
        }
        for id in &batch.touched {
            if let Some(n) = self.nodes.get_mut(id) {
                n.stamp = new_stamp;
            }
        }
        self.stamp = new_stamp;
        if batch.structural {
            self.structure_stamp = new_stamp;
        }
        self.log.extend(batch.changes.iter().cloned());
        self.trim_log();
        Ok((
            Applied {
                stamp: new_stamp,
                inserted: batch.inserted,
            },
            batch.changes,
        ))
    }

    fn trim_log(&mut self) {
        while let Some(front) = self.log.front() {
            let t = front.stamp;
            let group = self.log.iter().take_while(|c| c.stamp == t).count();
            if self.log.len() - group < MIN_LOG_CHANGES {
                break;
            }
            self.log.drain(..group);
            self.oldest = t;
        }
    }

    fn rollback(&mut self, undo: Vec<Undo>) {
        for u in undo.into_iter().rev() {
            match u {
                Undo::Insert { id, parent } => {
                    self.detach(parent, id);
                    self.nodes.remove(&id);
                }
                Undo::Update {
                    id,
                    payload,
                    vectors,
                } => {
                    if let Some(n) = self.nodes.get_mut(&id) {
                        if let Some(p) = payload {
                            n.payload = p;
                        }
                        if let Some(v) = vectors {
                            n.vectors = v;
                        }
                    }
                }
                Undo::Move {
                    id,
                    old_parent,
                    old_index,
                    new_parent,
                } => {
                    self.detach(new_parent, id);
                    self.attach(old_parent, id, old_index);
                    if let Some(n) = self.nodes.get_mut(&id) {
                        n.parent = old_parent;
                    }
                }
                Undo::Remove {
                    id,
                    parent,
                    index,
                    nodes,
                } => {
                    for (i, n) in nodes {
                        self.nodes.insert(i, n);
                    }
                    self.attach(parent, id, index);
                }
            }
        }
    }

    fn run_ops(&mut self, ops: Vec<Op>, stamp: Stamp, b: &mut Batch) -> Result<(), IndexError> {
        for (i, op) in ops.into_iter().enumerate() {
            match op {
                Op::Insert {
                    parent,
                    payload,
                    vectors,
                } => {
                    self.check_vectors(&vectors)?;
                    let p = match parent {
                        ParentRef::Root => None,
                        ParentRef::Entry(e) => {
                            if !self.nodes.contains_key(&e) {
                                return Err(IndexError::InvalidParent(format!(
                                    "unknown parent {e}"
                                )));
                            }
                            Some(e)
                        }
                        ParentRef::New(j) => match b.inserted_by_op.get(j).copied().flatten() {
                            Some(e) if j < i && self.nodes.contains_key(&e) => Some(e),
                            _ => {
                                return Err(IndexError::InvalidParent(format!(
                                    "invalid New({j}) reference at op {i}"
                                )))
                            }
                        },
                    };
                    let id = self.next_id;
                    self.next_id += 1;
                    self.nodes.insert(
                        id,
                        Node {
                            parent: p,
                            children: Vec::new(),
                            payload,
                            vectors,
                            stamp,
                        },
                    );
                    b.undo.push(Undo::Insert { id, parent: p });
                    self.attach(p, id, usize::MAX);
                    b.inserted_by_op[i] = Some(id);
                    b.inserted.push(id);
                    b.changes.push(Change {
                        stamp,
                        id,
                        kind: ChangeKind::Inserted,
                    });
                    b.structural = true;
                }
                Op::Update {
                    id,
                    payload,
                    vectors,
                } => {
                    if !self.nodes.contains_key(&id) {
                        return Err(IndexError::NotFound(id));
                    }
                    if let Some(v) = &vectors {
                        self.check_vectors(v)?;
                    }
                    let Some(n) = self.nodes.get_mut(&id) else {
                        return Err(IndexError::NotFound(id));
                    };
                    let old_payload = payload.map(|p| std::mem::replace(&mut n.payload, p));
                    let old_vectors = vectors.map(|v| std::mem::replace(&mut n.vectors, v));
                    b.undo.push(Undo::Update {
                        id,
                        payload: old_payload,
                        vectors: old_vectors,
                    });
                    b.touched.push(id);
                    b.changes.push(Change {
                        stamp,
                        id,
                        kind: ChangeKind::Updated,
                    });
                }
                Op::Move {
                    id,
                    parent,
                    position,
                } => {
                    let old_parent = match self.nodes.get(&id) {
                        Some(n) => n.parent,
                        None => return Err(IndexError::NotFound(id)),
                    };
                    if let Some(p) = parent {
                        if !self.nodes.contains_key(&p) {
                            return Err(IndexError::InvalidParent(format!("unknown parent {p}")));
                        }
                        let mut cur = Some(p);
                        while let Some(c) = cur {
                            if c == id {
                                return Err(IndexError::InvalidParent(format!(
                                    "target {p} lies within the subtree of {id}"
                                )));
                            }
                            cur = self.nodes.get(&c).and_then(|n| n.parent);
                        }
                    }
                    let old_index = self
                        .detach(old_parent, id)
                        .ok_or_else(|| corrupt("sibling list out of sync"))?;
                    let new_index = self
                        .attach(parent, id, position.unwrap_or(usize::MAX))
                        .ok_or_else(|| corrupt("missing move target"))?;
                    let _ = new_index;
                    if let Some(n) = self.nodes.get_mut(&id) {
                        n.parent = parent;
                    }
                    b.undo.push(Undo::Move {
                        id,
                        old_parent,
                        old_index,
                        new_parent: parent,
                    });
                    b.touched.push(id);
                    b.changes.push(Change {
                        stamp,
                        id,
                        kind: ChangeKind::Moved,
                    });
                    b.structural = true;
                }
                Op::Remove { id } => {
                    let parent = match self.nodes.get(&id) {
                        Some(n) => n.parent,
                        None => return Err(IndexError::NotFound(id)),
                    };
                    let ids = self.preorder_from(id);
                    let index = self
                        .detach(parent, id)
                        .ok_or_else(|| corrupt("sibling list out of sync"))?;
                    let nodes: Vec<(EntryId, Node)> = ids
                        .iter()
                        .filter_map(|x| self.nodes.remove(x).map(|n| (*x, n)))
                        .collect();
                    b.undo.push(Undo::Remove {
                        id,
                        parent,
                        index,
                        nodes,
                    });
                    for x in ids {
                        b.changes.push(Change {
                            stamp,
                            id: x,
                            kind: ChangeKind::Removed,
                        });
                    }
                    b.structural = true;
                }
            }
        }
        Ok(())
    }

    fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
        put_str(&mut out, &self.header.model_id);
        out.extend_from_slice(&(self.header.dimension as u64).to_le_bytes());
        out.push(dtype_code(self.header.storage_dtype));
        out.push(match self.header.pooling {
            Pooling::Mean => 0,
            Pooling::Cls => 1,
        });
        out.push(match self.header.normalization {
            Normalization::L2 => 0,
            Normalization::None => 1,
        });
        match &self.header.producing_profile {
            Some(p) => {
                out.push(1);
                put_str(&mut out, p);
            }
            None => out.push(0),
        }
        out.extend_from_slice(&self.stamp.to_le_bytes());
        out.extend_from_slice(&self.structure_stamp.to_le_bytes());
        out.extend_from_slice(&self.next_id.to_le_bytes());
        out.extend_from_slice(&(self.nodes.len() as u64).to_le_bytes());
        let mut stack: Vec<EntryId> = self.roots.iter().rev().copied().collect();
        while let Some(id) = stack.pop() {
            let Some(n) = self.nodes.get(&id) else {
                continue;
            };
            out.extend_from_slice(&id.to_le_bytes());
            out.extend_from_slice(&n.parent.unwrap_or(0).to_le_bytes());
            out.extend_from_slice(&n.stamp.to_le_bytes());
            out.extend_from_slice(&(n.payload.len() as u64).to_le_bytes());
            out.extend_from_slice(&n.payload);
            out.extend_from_slice(&(n.vectors.len() as u64).to_le_bytes());
            out.extend_from_slice(&n.vectors);
            stack.extend(n.children.iter().rev().copied());
        }
        let sum = checksum(&out);
        out.extend_from_slice(&sum.to_le_bytes());
        out
    }

    fn parse(data: &[u8], expected: &IndexHeader) -> Result<State, IndexError> {
        if data.len() < MAGIC.len() + 4 + 8 {
            return Err(corrupt("file too short"));
        }
        if &data[..MAGIC.len()] != MAGIC {
            return Err(corrupt("unknown magic"));
        }
        let (body, tail) = data.split_at(data.len() - 8);
        let mut stored = [0u8; 8];
        stored.copy_from_slice(tail);
        let version = u32::from_le_bytes([body[8], body[9], body[10], body[11]]);
        if version != FORMAT_VERSION {
            return Err(corrupt(format!("unsupported format version {version}")));
        }
        if checksum(body) != u64::from_le_bytes(stored) {
            return Err(corrupt("checksum mismatch"));
        }
        let mut r = Reader {
            buf: body,
            pos: 12,
        };
        let model_id = r.string()?;
        let dimension = usize::try_from(r.u64()?).map_err(|_| corrupt("dimension overflow"))?;
        let storage_dtype = dtype_from_code(r.u8()?)?;
        let pooling = match r.u8()? {
            0 => Pooling::Mean,
            1 => Pooling::Cls,
            _ => return Err(corrupt("unknown pooling")),
        };
        let normalization = match r.u8()? {
            0 => Normalization::L2,
            1 => Normalization::None,
            _ => return Err(corrupt("unknown normalization")),
        };
        let producing_profile = match r.u8()? {
            0 => None,
            1 => Some(r.string()?),
            _ => return Err(corrupt("bad profile flag")),
        };
        let found = IndexHeader {
            model_id,
            dimension,
            storage_dtype,
            pooling,
            normalization,
            producing_profile,
        };
        if !found.is_compatible(expected) {
            return Err(IndexError::ModelMismatch {
                expected: expected.clone(),
                found,
            });
        }
        let stamp = r.u64()?;
        let structure_stamp = r.u64()?;
        let next_id = r.u64()?;
        let count = r.u64()?;
        if structure_stamp > stamp {
            return Err(corrupt("structure stamp exceeds stamp"));
        }
        if count > (r.remaining() / MIN_ENTRY_BYTES) as u64 {
            return Err(corrupt("entry count exceeds file size"));
        }
        let vb = found.vector_bytes();
        let mut st = State::empty(found);
        st.stamp = stamp;
        st.structure_stamp = structure_stamp;
        st.next_id = next_id;
        st.oldest = stamp;
        for _ in 0..count {
            let id = r.u64()?;
            let parent_raw = r.u64()?;
            let estamp = r.u64()?;
            let payload = r.blob()?.to_vec();
            let vectors = r.blob()?.to_vec();
            if id == 0 || id >= next_id {
                return Err(corrupt(format!("invalid entry id {id}")));
            }
            if st.nodes.contains_key(&id) {
                return Err(corrupt(format!("duplicate entry id {id}")));
            }
            if estamp > stamp {
                return Err(corrupt("entry stamp exceeds stamp"));
            }
            let ok = if vb == 0 {
                vectors.is_empty()
            } else {
                vectors.len() % vb == 0
            };
            if !ok {
                return Err(corrupt(format!("invalid vector length for entry {id}")));
            }
            let parent = if parent_raw == 0 { None } else { Some(parent_raw) };
            if let Some(p) = parent {
                if !st.nodes.contains_key(&p) {
                    return Err(corrupt(format!("entry {id} has unknown or cyclic parent {p}")));
                }
            }
            st.nodes.insert(
                id,
                Node {
                    parent,
                    children: Vec::new(),
                    payload,
                    vectors,
                    stamp: estamp,
                },
            );
            st.attach(parent, id, usize::MAX);
        }
        if r.remaining() != 0 {
            return Err(corrupt("trailing bytes"));
        }
        Ok(st)
    }
}

fn put_str(out: &mut Vec<u8>, s: &str) {
    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
    out.extend_from_slice(s.as_bytes());
}

fn dtype_code(d: DType) -> u8 {
    match d {
        DType::F32 => 0,
        DType::F16 => 1,
        DType::BF16 => 2,
        DType::U8 => 3,
        DType::U32 => 4,
        DType::I64 => 5,
    }
}

fn dtype_from_code(c: u8) -> Result<DType, IndexError> {
    Ok(match c {
        0 => DType::F32,
        1 => DType::F16,
        2 => DType::BF16,
        3 => DType::U8,
        4 => DType::U32,
        5 => DType::I64,
        _ => return Err(corrupt("unknown dtype")),
    })
}

fn checksum(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in data {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], IndexError> {
        if n > self.remaining() {
            return Err(corrupt("unexpected end of data"));
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    fn u8(&mut self) -> Result<u8, IndexError> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, IndexError> {
        let mut a = [0u8; 4];
        a.copy_from_slice(self.take(4)?);
        Ok(u32::from_le_bytes(a))
    }

    fn u64(&mut self) -> Result<u64, IndexError> {
        let mut a = [0u8; 8];
        a.copy_from_slice(self.take(8)?);
        Ok(u64::from_le_bytes(a))
    }

    fn blob(&mut self) -> Result<&'a [u8], IndexError> {
        let n = usize::try_from(self.u64()?).map_err(|_| corrupt("length overflow"))?;
        self.take(n)
    }

    fn string(&mut self) -> Result<String, IndexError> {
        let n = self.u32()? as usize;
        let bytes = self.take(n)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| corrupt("invalid utf-8 string"))
    }
}

enum Slot {
    Unloaded,
    Ready(State),
    Mismatch(IndexHeader),
    Corrupt(String),
}

pub struct Index {
    path: PathBuf,
    expected: IndexHeader,
    slot: RwLock<Slot>,
    subscribers: Mutex<Vec<Sender<Change>>>,
}

pub struct IndexView<'a> {
    state: &'a State,
}

impl<'a> IndexView<'a> {
    pub fn header(&self) -> &IndexHeader {
        &self.state.header
    }

    pub fn stamp(&self) -> Stamp {
        self.state.stamp
    }

    pub fn structure_stamp(&self) -> Stamp {
        self.state.structure_stamp
    }

    pub fn len(&self) -> usize {
        self.state.nodes.len()
    }

    pub fn payload(&self, id: EntryId) -> Option<&[u8]> {
        self.state.nodes.get(&id).map(|n| n.payload.as_slice())
    }

    pub fn vectors(&self, id: EntryId) -> Option<&[u8]> {
        self.state.nodes.get(&id).map(|n| n.vectors.as_slice())
    }

    pub fn parent(&self, id: EntryId) -> Option<Option<EntryId>> {
        self.state.nodes.get(&id).map(|n| n.parent)
    }

    pub fn children(&self, parent: Option<EntryId>) -> &[EntryId] {
        self.state.siblings(parent)
    }

    pub fn entry_stamp(&self, id: EntryId) -> Option<Stamp> {
        self.state.nodes.get(&id).map(|n| n.stamp)
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = EntryId> + '_> {
        Box::new(self.state.nodes.keys().copied())
    }
}

fn read_guard(l: &RwLock<Slot>) -> RwLockReadGuard<'_, Slot> {
    l.read().unwrap_or_else(|e| e.into_inner())
}

fn write_guard(l: &RwLock<Slot>) -> RwLockWriteGuard<'_, Slot> {
    l.write().unwrap_or_else(|e| e.into_inner())
}

fn lock_subs(m: &Mutex<Vec<Sender<Change>>>) -> MutexGuard<'_, Vec<Sender<Change>>> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

impl Index {
    pub fn open(path: impl Into<PathBuf>, expected: IndexHeader) -> Index {
        Index {
            path: path.into(),
            expected,
            slot: RwLock::new(Slot::Unloaded),
            subscribers: Mutex::new(Vec::new()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn load(&self) -> Result<Slot, IndexError> {
        let data = match fs::read(&self.path) {
            Ok(d) => d,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(Slot::Ready(State::empty(self.expected.clone())))
            }
            Err(e) => return Err(IndexError::Io(e)),
        };
        match State::parse(&data, &self.expected) {
            Ok(s) => Ok(Slot::Ready(s)),
            Err(IndexError::ModelMismatch { found, .. }) => Ok(Slot::Mismatch(found)),
            Err(IndexError::Corrupt(m)) => Ok(Slot::Corrupt(m)),
            Err(e) => Err(e),
        }
    }

    fn ensure_loaded(&self) -> Result<(), IndexError> {
        if !matches!(*read_guard(&self.slot), Slot::Unloaded) {
            return Ok(());
        }
        let mut w = write_guard(&self.slot);
        if matches!(*w, Slot::Unloaded) {
            *w = self.load()?;
        }
        Ok(())
    }

    fn slot_error(&self, slot: &Slot) -> Option<IndexError> {
        match slot {
            Slot::Mismatch(found) => Some(IndexError::ModelMismatch {
                expected: self.expected.clone(),
                found: found.clone(),
            }),
            Slot::Corrupt(m) => Some(IndexError::Corrupt(m.clone())),
            _ => None,
        }
    }

    fn with_state<R>(&self, f: impl FnOnce(&State) -> R) -> Result<R, IndexError> {
        self.ensure_loaded()?;
        let g = read_guard(&self.slot);
        match &*g {
            Slot::Ready(s) => Ok(f(s)),
            Slot::Unloaded => Err(corrupt("index not loaded")),
            other => Err(self.slot_error(other).unwrap_or_else(|| corrupt("unavailable"))),
        }
    }

    pub fn header(&self) -> Result<IndexHeader, IndexError> {
        self.with_state(|s| s.header.clone())
    }

    pub fn get(&self, id: EntryId) -> Result<Option<Entry>, IndexError> {
        self.with_state(|s| {
            s.nodes.get(&id).map(|n| Entry {
                id,
                parent: n.parent,
                payload: n.payload.clone(),
                vectors: n.vectors.clone(),
                stamp: n.stamp,
            })
        })
    }

    pub fn children(&self, parent: Option<EntryId>) -> Result<Vec<EntryId>, IndexError> {
        self.with_state(|s| s.siblings(parent).to_vec())
    }

    pub fn len(&self) -> Result<usize, IndexError> {
        self.with_state(|s| s.nodes.len())
    }

    pub fn read<R>(&self, f: impl FnOnce(&IndexView<'_>) -> R) -> Result<R, IndexError> {
        self.with_state(|s| f(&IndexView { state: s }))
    }

    pub fn apply(&self, ops: Vec<Op>) -> Result<Applied, IndexError> {
        self.ensure_loaded()?;
        let mut g = write_guard(&self.slot);
        let state = match &mut *g {
            Slot::Ready(s) => s,
            Slot::Unloaded => return Err(corrupt("index not loaded")),
            other => {
                return Err(self
                    .slot_error(other)
                    .unwrap_or_else(|| corrupt("unavailable")))
            }
        };
        let (applied, changes) = state.apply_ops(ops)?;
        if !changes.is_empty() {
            let mut subs = lock_subs(&self.subscribers);
            subs.retain(|tx| changes.iter().all(|c| tx.send(c.clone()).is_ok()));
        }
        Ok(applied)
    }

    pub fn stamp(&self) -> Result<Stamp, IndexError> {
        self.with_state(|s| s.stamp)
    }

    pub fn structure_stamp(&self) -> Result<Stamp, IndexError> {
        self.with_state(|s| s.structure_stamp)
    }

    pub fn changes_since(&self, stamp: Stamp) -> Result<ChangeSet, IndexError> {
        self.with_state(|s| {
            if stamp >= s.stamp {
                return Ok(ChangeSet {
                    from: stamp,
                    to: s.stamp,
                    changes: Vec::new(),
                });
            }
            if stamp < s.oldest {
                return Err(IndexError::HistoryTruncated { oldest: s.oldest });
            }
            let start = s.log.partition_point(|c| c.stamp <= stamp);
            Ok(ChangeSet {
                from: stamp,
                to: s.stamp,
                changes: s.log.iter().skip(start).cloned().collect(),
            })
        })?
    }

    pub fn subscribe(&self) -> Receiver<Change> {
        let (tx, rx) = channel();
        lock_subs(&self.subscribers).push(tx);
        rx
    }

    pub fn flush(&self) -> Result<(), IndexError> {
        let g = write_guard(&self.slot);
        let state = match &*g {
            Slot::Unloaded => return Ok(()),
            Slot::Ready(s) => s,
            other => {
                return Err(self
                    .slot_error(other)
                    .unwrap_or_else(|| corrupt("unavailable")))
            }
        };
        let bytes = state.serialize();
        let mut tmp_name = self
            .path
            .file_name()
            .map(|n| n.to_os_string())
            .unwrap_or_default();
        tmp_name.push(".tmp");
        let tmp = self.path.with_file_name(tmp_name);
        let result = (|| -> io::Result<()> {
            let mut f = fs::File::create(&tmp)?;
            f.write_all(&bytes)?;
            f.sync_all()?;
            drop(f);
            fs::rename(&tmp, &self.path)
        })();
        if let Err(e) = result {
            let _ = fs::remove_file(&tmp);
            return Err(IndexError::Io(e));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/index.rs"]
mod tests;
