**Stack**

**Model-Loading:**
- `safetensors` für ColBERT-Weights laden
- `candle` für Compute (CUDA-Abstraktion, minimal Overhead)
- `tokenizers` crate für BPE/Tokenization

**CUDA/GPU:**
- `candle-cuda` für Forward-Passes
- `cudarc` optional für Custom-Kernel (MaxSim)
- GTX 1660 reicht (6GB VRAM für residentes Modell + Index-Batch)

**Concurrency/Streaming:**
- `tokio` für Async
- `crossbeam::queue` oder `parking_lot` für Lock-free Ring-Buffer
- Threads für CPU-Tokenizer parallel zu GPU-Compute

**Index:**
- `arrow2` oder `parquet` für Persistierung
- Columnar Layout (Embeddings pro Column)
- Mmap für Read-Heavy Zugriff

**Architecture:**
```
CPU-Thread: Tokenizer + Doc-Stream → Query-Queue
GPU-Thread: Sliding-Window-Embedding-Kernel → Index-Buffer
GPU-Thread: MaxSim gegen Kandidaten
Ring-Buffer: Entkoppelt Producer/Consumer
```