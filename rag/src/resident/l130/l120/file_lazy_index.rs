use std::path::PathBuf;
use std::sync::Arc;

use super::l110::l100::l090::a::entry_store::{EntryStore, PayloadChecksums};
use super::l110::l100::l090::a::persistent_save::entry_payload::compute_checksum_parts;
use super::l110::l100::l090::l080::a::index_format::IndexHeader;
use super::l110::l100::l090::l080::a::parquet_decoder::{ChecksumFn, ParquetDecoder};
use super::l110::l100::l090::l080::l070::l060::l050::l040::index_residency::IndexSlot;
use super::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::index_open::{
    LazyIndexFile, OpenOptions,
};
use super::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::Index;
use super::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::page_advice::AdviceSlot;
use super::manager::LazyIndex;

/// Opens the index file on the first successful `get` and then always returns the same instance.
pub struct FileLazyIndex {
    path: PathBuf,
    header: IndexHeader,
    open: OpenOptions,
    slot: IndexSlot,
    advice: Arc<AdviceSlot>,
}

impl FileLazyIndex {
    pub fn new(path: PathBuf, header: IndexHeader, open: OpenOptions, slot: IndexSlot) -> Self {
        Self {
            path,
            header,
            open,
            slot,
            advice: Arc::new(AdviceSlot::default()),
        }
    }

    /// Binds the slot that receives the hint target of the mapped file.
    pub fn with_advice_slot(mut self, advice: Arc<AdviceSlot>) -> Self {
        self.advice = advice;
        self
    }

    pub fn slot(&self) -> &IndexSlot {
        &self.slot
    }

    fn build(&self) -> Result<Arc<dyn Index>, String> {
        let checksum: ChecksumFn = Arc::new(compute_checksum_parts);
        let decoder = Arc::new(ParquetDecoder::new(self.header.clone(), checksum));
        let checksums = Arc::new(PayloadChecksums::new(Arc::clone(&decoder)));
        let file = if self.path.exists() {
            let file = LazyIndexFile::new(&self.path, decoder, self.open.clone())
                .with_advice_slot(Arc::clone(&self.advice));
            file.ensure_open().map_err(|e| e.to_string())?;
            Some(Arc::new(file))
        } else {
            None
        };
        Ok(Arc::new(EntryStore::new(self.header.clone(), file, checksums)))
    }
}

impl LazyIndex for FileLazyIndex {
    fn get(&self) -> Result<Arc<dyn Index>, String> {
        if let Some(index) = self.slot.get() {
            return Ok(Arc::clone(index));
        }
        let opened = self.build()?;
        let _ = self.slot.set(opened);
        self.slot
            .get()
            .cloned()
            .ok_or_else(|| "index slot is empty after opening".to_string())
    }
}

#[cfg(test)]
#[path = "test/file_lazy_index.rs"]
mod tests;
