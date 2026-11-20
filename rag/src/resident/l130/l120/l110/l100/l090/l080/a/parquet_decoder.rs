use std::io::Cursor;
use std::ops::Range;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use arrow::array::{Array, FixedSizeListArray, Float16Array, StringArray, UInt64Array};
use arrow::error::ArrowError;
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::{
    ArrowReaderMetadata, ArrowReaderOptions, ParquetRecordBatchReader,
    ParquetRecordBatchReaderBuilder,
};
use parquet::arrow::ProjectionMask;
use parquet::errors::ParquetError;
use parquet::file::metadata::ParquetMetaData;
use parquet::file::reader::{ChunkReader, Length};
use tokio_util::bytes::Bytes;

use super::index_format::{
    check_column_schema, FormatError, IndexHeader, COL_CHANGE_STAMP, COL_CHAR_END, COL_CHAR_START,
    COL_CHECKSUM, COL_ENTRY_ID, COL_TARGET_FILE, COL_VECTOR,
};
use super::meta_codec::encode_meta_parts;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::index_open::{
    DecodedBatch, FileInfo, FormatDecoder,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::SharedBytes;

pub const READ_BATCH_ROWS: usize = 4096;

pub type ChecksumFn = Arc<dyn Fn(&[u8], &str, u64, u64) -> u64 + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    #[error("unreadable file: {0}")]
    Footer(String),
    #[error("{0}")]
    Format(FormatError),
    #[error("invalid data: {0}")]
    Data(String),
    #[error("checksum mismatch at row {row} (entry {entry_id}): stored {stored:016x}, computed {computed:016x}")]
    Checksum {
        row: u64,
        entry_id: u64,
        stored: u64,
        computed: u64,
    },
}

pub struct ParquetDecoder {
    expected: IndexHeader,
    checksum: ChecksumFn,
}

impl ParquetDecoder {
    pub fn new(expected: IndexHeader, checksum: ChecksumFn) -> ParquetDecoder {
        ParquetDecoder { expected, checksum }
    }

    pub fn expected(&self) -> &IndexHeader {
        &self.expected
    }

    pub fn read_header(&self, bytes: &[u8]) -> Result<IndexHeader, DecodeError> {
        guarded(|| load_header(bytes).map(|(_, header)| header))
    }

    pub fn read_stored_checksums(&self, bytes: &[u8]) -> Result<Vec<(u64, u64)>, DecodeError> {
        guarded(|| {
            let (meta, _) = load_header(bytes)?;
            read_pairs(bytes, &meta, COL_ENTRY_ID, COL_CHECKSUM)
        })
    }

    fn read_info_inner(&self, bytes: &[u8]) -> Result<FileInfo, DecodeError> {
        let (meta, header) = load_header(bytes)?;
        header
            .check_binding(&self.expected)
            .map_err(DecodeError::Format)?;
        let schema_version = u32::try_from(header.schema_version).map_err(|_| {
            DecodeError::Data(format!(
                "schema version {} does not fit into 32 bits",
                header.schema_version
            ))
        })?;
        let counts = group_rows(meta.metadata())?;
        let rows = total_rows(&counts)?;
        let file_rows = u64::try_from(meta.metadata().file_metadata().num_rows())
            .map_err(|_| DecodeError::Data("negative file row count".into()))?;
        if rows != file_rows {
            return Err(DecodeError::Data(format!(
                "row groups hold {rows} rows, file declares {file_rows}"
            )));
        }
        Ok(FileInfo {
            schema_version,
            model_id: header.model_id,
            model_revision: header.model_revision,
            row_groups: counts.len(),
            rows,
            stored_max_stamp: Some(header.max_stamp),
        })
    }

    fn read_ids_stamps_inner(
        &self,
        bytes: &[u8],
        info: &FileInfo,
    ) -> Result<Vec<(u64, u64)>, DecodeError> {
        let meta = load_metadata(bytes)?;
        let pairs = read_pairs(bytes, &meta, COL_ENTRY_ID, COL_CHANGE_STAMP)?;
        if pairs.len() as u64 != info.rows {
            return Err(DecodeError::Data(format!(
                "file holds {} rows, expected {}",
                pairs.len(),
                info.rows
            )));
        }
        Ok(pairs)
    }

    fn decode_rows_inner(
        &self,
        bytes: &[u8],
        info: &FileInfo,
        start: u64,
        end: u64,
    ) -> Result<DecodedBatch, DecodeError> {
        if start > end || end > info.rows {
            return Err(DecodeError::Data(format!(
                "row range {start}..{end} outside of {} rows",
                info.rows
            )));
        }
        if start == end {
            return Ok(DecodedBatch {
                vectors: Vec::new(),
                metas: Vec::new(),
            });
        }
        let meta = load_metadata(bytes)?;
        let counts = group_rows(meta.metadata())?;
        let mut groups = Vec::new();
        let mut first_row = None;
        let mut cursor = 0u64;
        for (group, &count) in counts.iter().enumerate() {
            let group_end = checked_add(cursor, count)?;
            if group_end > start && cursor < end {
                first_row.get_or_insert(cursor);
                groups.push(group);
            }
            cursor = group_end;
        }
        let Some(mut position) = first_row.filter(|_| cursor >= end) else {
            return Err(DecodeError::Data(format!(
                "row range {start}..{end} outside of the {cursor} rows in the file"
            )));
        };

        let roots = root_indices(
            &meta,
            &[COL_VECTOR, COL_TARGET_FILE, COL_CHAR_START, COL_CHAR_END],
        )?;
        let reader = open_reader(bytes, &meta, groups, &roots)?;
        let wanted = usize::try_from(end - start)
            .map_err(|_| DecodeError::Data("row range too large".into()))?;
        let capacity = wanted.min(READ_BATCH_ROWS);
        let mut vectors = Vec::with_capacity(capacity);
        let mut metas = Vec::with_capacity(capacity);
        let mut buffer = Vec::with_capacity(self.expected.vector_bytes());

        for batch in reader {
            let batch = batch.map_err(batch_error)?;
            let rows = batch.num_rows() as u64;
            let low = position.max(start);
            let high = checked_add(position, rows)?.min(end);
            if low < high {
                let vector = VectorColumn::new(&batch, self.expected.dimension)?;
                let files = column::<StringArray>(&batch, COL_TARGET_FILE)?;
                let starts = column::<UInt64Array>(&batch, COL_CHAR_START)?;
                let ends = column::<UInt64Array>(&batch, COL_CHAR_END)?;
                for row in (low - position) as usize..(high - position) as usize {
                    buffer.clear();
                    vector.append(row, &mut buffer)?;
                    vectors.push(SharedBytes::from(buffer.as_slice()));
                    metas.push(SharedBytes::from(encode_meta_parts(
                        files.value(row),
                        starts.value(row),
                        ends.value(row),
                    )));
                }
            }
            position = checked_add(position, rows)?;
            if position >= end {
                break;
            }
        }
        if vectors.len() != wanted {
            return Err(DecodeError::Data(format!(
                "decoded {} rows, expected {wanted}",
                vectors.len()
            )));
        }
        Ok(DecodedBatch { vectors, metas })
    }

    fn verify_inner(&self, bytes: &[u8], info: &FileInfo) -> Result<(), DecodeError> {
        let (meta, header) = load_header(bytes)?;
        let counts = group_rows(meta.metadata())?;
        let declared = total_rows(&counts)?;
        if declared != info.rows {
            return Err(DecodeError::Data(format!(
                "row groups hold {declared} rows, expected {}",
                info.rows
            )));
        }
        let roots = root_indices(
            &meta,
            &[
                COL_ENTRY_ID,
                COL_CHANGE_STAMP,
                COL_VECTOR,
                COL_TARGET_FILE,
                COL_CHAR_START,
                COL_CHAR_END,
                COL_CHECKSUM,
            ],
        )?;

        let mut row = 0u64;
        let mut previous: Option<u64> = None;
        let mut buffer = Vec::with_capacity(self.expected.vector_bytes());
        for (group, &count) in counts.iter().enumerate() {
            let group_first = row;
            let reader = open_reader(bytes, &meta, vec![group], &roots)?;
            for batch in reader {
                let batch = batch.map_err(batch_error)?;
                let ids = column::<UInt64Array>(&batch, COL_ENTRY_ID)?;
                let stamps = column::<UInt64Array>(&batch, COL_CHANGE_STAMP)?;
                let vector = VectorColumn::new(&batch, self.expected.dimension)?;
                let files = column::<StringArray>(&batch, COL_TARGET_FILE)?;
                let starts = column::<UInt64Array>(&batch, COL_CHAR_START)?;
                let ends = column::<UInt64Array>(&batch, COL_CHAR_END)?;
                let sums = column::<UInt64Array>(&batch, COL_CHECKSUM)?;
                for i in 0..batch.num_rows() {
                    let id = ids.value(i);
                    let stamp = stamps.value(i);
                    buffer.clear();
                    vector.append(i, &mut buffer)?;
                    if let Some(prev) = previous {
                        if id <= prev {
                            return Err(DecodeError::Data(format!(
                                "entry ids not strictly ascending at row {row}: {id} after {prev}"
                            )));
                        }
                    }
                    previous = Some(id);
                    if stamp == 0 || stamp > header.max_stamp {
                        return Err(DecodeError::Data(format!(
                            "change stamp {stamp} at row {row} outside 1..={}",
                            header.max_stamp
                        )));
                    }
                    let stored = sums.value(i);
                    let computed =
                        (self.checksum)(&buffer, files.value(i), starts.value(i), ends.value(i));
                    if stored != computed {
                        return Err(DecodeError::Checksum {
                            row,
                            entry_id: id,
                            stored,
                            computed,
                        });
                    }
                    row += 1;
                }
            }
            if row - group_first != count {
                return Err(DecodeError::Data(format!(
                    "row group {group} holds {} rows, declared {count}",
                    row - group_first
                )));
            }
        }
        Ok(())
    }
}

impl FormatDecoder for ParquetDecoder {
    fn read_info(&self, bytes: &[u8]) -> Result<FileInfo, String> {
        guarded(|| self.read_info_inner(bytes)).map_err(|e| e.to_string())
    }

    fn read_ids_stamps(&self, bytes: &[u8], info: &FileInfo) -> Result<Vec<(u64, u64)>, String> {
        guarded(|| self.read_ids_stamps_inner(bytes, info)).map_err(|e| e.to_string())
    }

    fn decode_rows(
        &self,
        bytes: &[u8],
        info: &FileInfo,
        start: u64,
        end: u64,
    ) -> Result<DecodedBatch, String> {
        guarded(|| self.decode_rows_inner(bytes, info, start, end)).map_err(|e| e.to_string())
    }

    fn verify(&self, bytes: &[u8], info: &FileInfo) -> Result<(), String> {
        guarded(|| self.verify_inner(bytes, info)).map_err(|e| e.to_string())
    }
}

fn guarded<T>(f: impl FnOnce() -> Result<T, DecodeError>) -> Result<T, DecodeError> {
    catch_unwind(AssertUnwindSafe(f))
        .unwrap_or_else(|_| Err(DecodeError::Footer("malformed file".into())))
}

fn checked_add(a: u64, b: u64) -> Result<u64, DecodeError> {
    a.checked_add(b)
        .ok_or_else(|| DecodeError::Data("row count overflow".into()))
}

fn batch_error(e: ArrowError) -> DecodeError {
    DecodeError::Data(format!("unreadable row data: {e}"))
}

fn reader_error(e: ParquetError) -> DecodeError {
    DecodeError::Data(format!("unreadable row data: {e}"))
}

fn load_metadata(bytes: &[u8]) -> Result<ArrowReaderMetadata, DecodeError> {
    ArrowReaderMetadata::load(&SliceSource(bytes), ArrowReaderOptions::new())
        .map_err(|e| DecodeError::Footer(e.to_string()))
}

fn load_header(bytes: &[u8]) -> Result<(ArrowReaderMetadata, IndexHeader), DecodeError> {
    let meta = load_metadata(bytes)?;
    let pairs: Vec<(String, String)> = meta
        .metadata()
        .file_metadata()
        .key_value_metadata()
        .map(|kvs| {
            kvs.iter()
                .filter_map(|kv| Some((kv.key.clone(), kv.value.clone()?)))
                .collect()
        })
        .unwrap_or_default();
    let header = IndexHeader::from_kv(&pairs).map_err(DecodeError::Format)?;
    check_column_schema(meta.schema().as_ref(), header.dimension).map_err(DecodeError::Format)?;
    Ok((meta, header))
}

fn group_rows(meta: &ParquetMetaData) -> Result<Vec<u64>, DecodeError> {
    meta.row_groups()
        .iter()
        .enumerate()
        .map(|(i, group)| {
            u64::try_from(group.num_rows())
                .map_err(|_| DecodeError::Data(format!("row group {i} has a negative row count")))
        })
        .collect()
}

fn total_rows(counts: &[u64]) -> Result<u64, DecodeError> {
    counts.iter().try_fold(0u64, |sum, &n| checked_add(sum, n))
}

fn root_indices(meta: &ArrowReaderMetadata, names: &[&str]) -> Result<Vec<usize>, DecodeError> {
    let fields = meta.parquet_schema().root_schema().get_fields();
    names
        .iter()
        .map(|name| {
            fields
                .iter()
                .position(|field| field.name() == *name)
                .ok_or_else(|| DecodeError::Data(format!("missing column {name}")))
        })
        .collect()
}

fn chunk_ranges(
    meta: &ParquetMetaData,
    groups: &[usize],
    roots: &[usize],
    file_len: u64,
) -> Result<Vec<Range<u64>>, DecodeError> {
    let schema = meta.file_metadata().schema_descr();
    let mut ranges = Vec::new();
    for &group in groups {
        let columns = meta
            .row_groups()
            .get(group)
            .ok_or_else(|| DecodeError::Data(format!("row group {group} does not exist")))?
            .columns();
        for (leaf, column) in columns.iter().enumerate() {
            if leaf >= schema.num_columns() || !roots.contains(&schema.get_column_root_idx(leaf)) {
                continue;
            }
            let (start, length) = column.byte_range();
            let end = start
                .checked_add(length)
                .filter(|&end| end <= file_len)
                .ok_or_else(|| {
                    DecodeError::Data(format!("column chunk {leaf} of group {group} is outside the file"))
                })?;
            ranges.push(start..end);
        }
    }
    Ok(ranges)
}

fn open_reader(
    bytes: &[u8],
    meta: &ArrowReaderMetadata,
    groups: Vec<usize>,
    roots: &[usize],
) -> Result<ParquetRecordBatchReader, DecodeError> {
    let ranges = chunk_ranges(meta.metadata(), &groups, roots, bytes.len() as u64)?;
    let source = CopiedRanges::copy(bytes, ranges)?;
    let mask = ProjectionMask::roots(meta.parquet_schema(), roots.iter().copied());
    ParquetRecordBatchReaderBuilder::new_with_metadata(source, meta.clone())
        .with_projection(mask)
        .with_row_groups(groups)
        .with_batch_size(READ_BATCH_ROWS)
        .build()
        .map_err(reader_error)
}

fn read_pairs(
    bytes: &[u8],
    meta: &ArrowReaderMetadata,
    first: &str,
    second: &str,
) -> Result<Vec<(u64, u64)>, DecodeError> {
    let roots = root_indices(meta, &[first, second])?;
    let groups: Vec<usize> = (0..meta.metadata().num_row_groups()).collect();
    let mut pairs = Vec::new();
    if groups.is_empty() {
        return Ok(pairs);
    }
    for batch in open_reader(bytes, meta, groups, &roots)? {
        let batch = batch.map_err(batch_error)?;
        let firsts = column::<UInt64Array>(&batch, first)?;
        let seconds = column::<UInt64Array>(&batch, second)?;
        pairs.extend(firsts.values().iter().copied().zip(seconds.values().iter().copied()));
    }
    Ok(pairs)
}

fn column<'a, A: Array + 'static>(batch: &'a RecordBatch, name: &str) -> Result<&'a A, DecodeError> {
    let array = batch
        .column_by_name(name)
        .ok_or_else(|| DecodeError::Data(format!("missing column {name}")))?;
    let typed = array
        .as_any()
        .downcast_ref::<A>()
        .ok_or_else(|| DecodeError::Data(format!("column {name} has an unexpected type")))?;
    if typed.null_count() > 0 {
        return Err(DecodeError::Data(format!("column {name} contains nulls")));
    }
    Ok(typed)
}

struct VectorColumn<'a> {
    list: &'a FixedSizeListArray,
    values: &'a Float16Array,
    dimension: usize,
}

impl<'a> VectorColumn<'a> {
    fn new(batch: &'a RecordBatch, dimension: usize) -> Result<Self, DecodeError> {
        let list = column::<FixedSizeListArray>(batch, COL_VECTOR)?;
        if usize::try_from(list.value_length()) != Ok(dimension) {
            return Err(DecodeError::Data(format!(
                "vector list size {} differs from dimension {dimension}",
                list.value_length()
            )));
        }
        let values = list
            .values()
            .as_any()
            .downcast_ref::<Float16Array>()
            .ok_or_else(|| DecodeError::Data("vector elements are not Float16".into()))?;
        if values.null_count() > 0 {
            return Err(DecodeError::Data("vector elements contain nulls".into()));
        }
        Ok(VectorColumn {
            list,
            values,
            dimension,
        })
    }

    fn append(&self, row: usize, out: &mut Vec<u8>) -> Result<(), DecodeError> {
        let offset = usize::try_from(self.list.value_offset(row))
            .map_err(|_| DecodeError::Data("negative vector offset".into()))?;
        let end = offset
            .checked_add(self.dimension)
            .ok_or_else(|| DecodeError::Data("vector offset overflow".into()))?;
        let elements = self
            .values
            .values()
            .get(offset..end)
            .ok_or_else(|| DecodeError::Data(format!("vector at row {row} is out of range")))?;
        out.extend(elements.iter().flat_map(|v| v.to_bits().to_le_bytes()));
        Ok(())
    }
}

struct SliceSource<'a>(&'a [u8]);

impl Length for SliceSource<'_> {
    fn len(&self) -> u64 {
        self.0.len() as u64
    }
}

impl<'a> ChunkReader for SliceSource<'a> {
    type T = Cursor<&'a [u8]>;

    fn get_read(&self, start: u64) -> Result<Self::T, ParquetError> {
        let tail = usize::try_from(start)
            .ok()
            .and_then(|s| self.0.get(s..))
            .ok_or_else(|| outside(start, 0))?;
        Ok(Cursor::new(tail))
    }

    fn get_bytes(&self, start: u64, length: usize) -> Result<Bytes, ParquetError> {
        let end = start.checked_add(length as u64);
        let slice = usize::try_from(start)
            .ok()
            .zip(end.and_then(|e| usize::try_from(e).ok()))
            .and_then(|(s, e)| self.0.get(s..e))
            .ok_or_else(|| outside(start, length))?;
        Ok(Bytes::copy_from_slice(slice))
    }
}

struct CopiedRanges {
    file_len: u64,
    parts: Vec<(u64, Bytes)>,
}

impl CopiedRanges {
    fn copy(bytes: &[u8], mut ranges: Vec<Range<u64>>) -> Result<CopiedRanges, DecodeError> {
        ranges.retain(|r| r.start < r.end);
        ranges.sort_by_key(|r| r.start);
        let mut merged: Vec<Range<u64>> = Vec::new();
        for range in ranges {
            match merged.last_mut() {
                Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
                _ => merged.push(range),
            }
        }
        let parts = merged
            .into_iter()
            .map(|range| {
                let slice = usize::try_from(range.start)
                    .ok()
                    .zip(usize::try_from(range.end).ok())
                    .and_then(|(s, e)| bytes.get(s..e))
                    .ok_or_else(|| DecodeError::Data("column chunk is outside the file".into()))?;
                Ok((range.start, Bytes::copy_from_slice(slice)))
            })
            .collect::<Result<Vec<_>, DecodeError>>()?;
        Ok(CopiedRanges {
            file_len: bytes.len() as u64,
            parts,
        })
    }

    fn locate(&self, start: u64, length: usize) -> Result<(&Bytes, usize), ParquetError> {
        let index = self.parts.partition_point(|(s, _)| *s <= start);
        let (part_start, part) = index
            .checked_sub(1)
            .and_then(|i| self.parts.get(i))
            .ok_or_else(|| outside(start, length))?;
        let offset = usize::try_from(start - part_start)
            .ok()
            .filter(|&o| o <= part.len() && length <= part.len() - o)
            .ok_or_else(|| outside(start, length))?;
        Ok((part, offset))
    }
}

impl Length for CopiedRanges {
    fn len(&self) -> u64 {
        self.file_len
    }
}

impl ChunkReader for CopiedRanges {
    type T = Cursor<Bytes>;

    fn get_read(&self, start: u64) -> Result<Self::T, ParquetError> {
        let (part, offset) = self.locate(start, 0)?;
        Ok(Cursor::new(part.slice(offset..)))
    }

    fn get_bytes(&self, start: u64, length: usize) -> Result<Bytes, ParquetError> {
        if length == 0 {
            return Ok(Bytes::new());
        }
        let (part, offset) = self.locate(start, length)?;
        Ok(part.slice(offset..offset + length))
    }
}

fn outside(start: u64, length: usize) -> ParquetError {
    ParquetError::General(format!("read of {length} bytes at {start} is outside the available data"))
}

#[cfg(test)]
#[path = "test/parquet_decoder.rs"]
mod tests;
