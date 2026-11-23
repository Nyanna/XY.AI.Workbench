use super::*;
use std::mem::{align_of, offset_of, size_of};
use std::sync::{Mutex, OnceLock};
use std::thread;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::region_handle::Generation;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
fn handle(len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::Workspace, 4096, len, Generation(3)).unwrap()
}
struct Capture {
    records: Mutex<Vec<String>>,
}
impl log::Log for Capture {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }
    fn log(&self, record: &log::Record) {
        if record.level() == log::Level::Warn {
            self.records.lock().unwrap().push(record.args().to_string());
        }
    }
    fn flush(&self) {}
}
static CAPTURE: Capture = Capture {
    records: Mutex::new(Vec::new()),
};
/// False when another test already owns the global logger.
fn capture_installed() -> bool {
    static INSTALLED: OnceLock<bool> = OnceLock::new();
    *INSTALLED
        .get_or_init(|| {
            let ok = log::set_logger(&CAPTURE).is_ok();
            if ok && log::max_level() < log::LevelFilter::Warn {
                log::set_max_level(log::LevelFilter::Warn);
            }
            ok
        })
}
fn captured(capacity: u64) -> usize {
    let needle = format!("capacity {capacity},");
    CAPTURE.records.lock().unwrap().iter().filter(|m| m.contains(&needle)).count()
}
#[test]
fn layout_is_32_bytes_with_fixed_offsets() {
    assert_eq!(size_of::< WriteState > (), STATE_SIZE);
    assert_eq!(align_of::< WriteState > (), 8);
    assert_eq!(offset_of!(WriteState, ptr), 0);
    assert_eq!(offset_of!(WriteState, overflow_min), 8);
    assert_eq!(offset_of!(WriteState, capacity), 16);
    assert_eq!(offset_of!(WriteState, overflow_flag), 24);
    assert_eq!(offset_of!(WriteState, logged), 28);
    assert_eq!(WARP_SIZE, 32);
}
#[test]
fn init_image_and_snapshot_from_image_agree_with_new() {
    let image = WriteState::init_image(1234);
    assert_eq!(& image[0..8], & 0u64.to_le_bytes());
    assert_eq!(& image[8..16], & u64::MAX.to_le_bytes());
    assert_eq!(& image[16..24], & 1234u64.to_le_bytes());
    assert_eq!(& image[24..32], & [0u8; 8]);
    let from_image = WriteState::snapshot_from_image(&image);
    assert_eq!(from_image, WriteState::new(1234).snapshot());
    assert_eq!(
        from_image, StateSnapshot { ptr : 0, overflow_min : NO_OVERFLOW, capacity : 1234,
        overflow : false }
    );
}
#[test]
fn snapshot_from_image_reads_device_fields() {
    let mut image = [0u8; STATE_SIZE];
    image[0..8].copy_from_slice(&900u64.to_le_bytes());
    image[8..16].copy_from_slice(&640u64.to_le_bytes());
    image[16..24].copy_from_slice(&700u64.to_le_bytes());
    image[24..28].copy_from_slice(&1u32.to_le_bytes());
    let s = WriteState::snapshot_from_image(&image);
    assert_eq!(
        s, StateSnapshot { ptr : 900, overflow_min : 640, capacity : 700, overflow : true
        }
    );
    assert_eq!(s.valid_len(), 640);
}
#[test]
fn single_claims_form_a_gapless_prefix_until_overflow() {
    let state = WriteState::new(100);
    assert_eq!(state.claim(30), Claim::Ok { offset : 0 });
    assert_eq!(state.claim(30), Claim::Ok { offset : 30 });
    assert_eq!(state.claim(30), Claim::Ok { offset : 60 });
    assert!(! state.snapshot().overflow);
    assert_eq!(state.claim(20), Claim::Invalid);
    assert_eq!(state.claim(5), Claim::Invalid);
    assert_eq!(state.claim(0), Claim::Invalid);
    let s = state.snapshot();
    assert!(s.overflow);
    assert_eq!(s.ptr, 115);
    assert_eq!(s.overflow_min, 90);
    assert_eq!(s.valid_len(), 90);
}
#[test]
fn claim_filling_capacity_exactly_succeeds() {
    let state = WriteState::new(64);
    assert_eq!(state.claim(64), Claim::Ok { offset : 0 });
    let s = state.snapshot();
    assert!(! s.overflow);
    assert_eq!(s.valid_len(), 64);
    assert_eq!(state.claim(0), Claim::Ok { offset : 64 });
    assert!(! state.snapshot().overflow);
    assert_eq!(state.claim(1), Claim::Invalid);
    let s = state.snapshot();
    assert_eq!(s.overflow_min, 64);
    assert_eq!(s.valid_len(), 64);
}
#[test]
fn zero_length_claim_succeeds_while_pointer_is_within_capacity() {
    let state = WriteState::new(10);
    assert_eq!(state.claim(0), Claim::Ok { offset : 0 });
    assert_eq!(state.claim(10), Claim::Ok { offset : 0 });
    assert_eq!(state.claim(0), Claim::Ok { offset : 10 });
    assert_eq!(WriteState::new(0).claim(0), Claim::Ok { offset : 0 });
    assert_eq!(WriteState::new(0).claim(1), Claim::Invalid);
}
#[test]
fn pointer_wrap_is_overflow_and_valid_len_stays_bounded() {
    let state = WriteState::new(10);
    assert_eq!(state.claim(u64::MAX), Claim::Invalid);
    assert_eq!(state.snapshot().ptr, u64::MAX);
    assert_eq!(state.claim(1), Claim::Invalid);
    let s = state.snapshot();
    assert_eq!(s.ptr, 0);
    assert!(s.overflow);
    assert_eq!(s.overflow_min, 0);
    assert!(s.valid_len() <= s.capacity);
}
#[test]
fn group_overflowing_mid_group_keeps_leading_members_valid() {
    let state = WriteState::new(100);
    assert_eq!(state.claim(10), Claim::Ok { offset : 0 });
    let lens = [30, 30, 40, 10];
    let mut out = [0u64; 4];
    state.claim_group(&lens, &mut out).unwrap();
    assert_eq!(out, [10, 40, INVALID_OFFSET, INVALID_OFFSET]);
    let s = state.snapshot();
    assert!(s.overflow);
    assert_eq!(s.ptr, 120);
    assert_eq!(s.overflow_min, 70);
    assert_eq!(s.valid_len(), 70);
}
#[test]
fn group_within_capacity_gets_exclusive_prefix_offsets() {
    let state = WriteState::new(100);
    let mut out = [0u64; 5];
    state.claim_group(&[5, 0, 20, 0, 75], &mut out).unwrap();
    assert_eq!(out, [0, 5, 5, 25, 25]);
    let s = state.snapshot();
    assert!(! s.overflow);
    assert_eq!(s.ptr, 100);
    assert_eq!(s.valid_len(), 100);
}
#[test]
fn group_after_overflow_fails_completely() {
    let state = WriteState::new(10);
    assert_eq!(state.claim(11), Claim::Invalid);
    let mut out = [0u64; 2];
    state.claim_group(&[1, 2], &mut out).unwrap();
    assert_eq!(out, [INVALID_OFFSET; 2]);
    assert_eq!(state.snapshot().overflow_min, 0);
}
#[test]
fn group_with_saturating_total_fails_without_panic() {
    let state = WriteState::new(10);
    let mut out = [0u64; 3];
    state.claim_group(&[u64::MAX, u64::MAX, 5], &mut out).unwrap();
    assert_eq!(out, [INVALID_OFFSET; 3]);
    let s = state.snapshot();
    assert!(s.overflow);
    assert_eq!(s.overflow_min, 0);
    assert!(s.valid_len() <= 10);
}
#[test]
fn group_length_mismatch_leaves_state_unchanged() {
    let state = WriteState::new(50);
    state.claim(7);
    let before = state.snapshot();
    let mut out = [123u64; 2];
    let err = state.claim_group(&[1, 2, 3], &mut out).unwrap_err();
    assert_eq!(err, ClaimError::LengthMismatch { lens : 3, out : 2 });
    assert_eq!(out, [123, 123]);
    assert_eq!(state.snapshot(), before);
    assert!(err.to_string().contains('3'));
}
#[test]
fn empty_group_changes_nothing() {
    let state = WriteState::new(50);
    state.claim(7);
    let before = state.snapshot();
    state.claim_group(&[], &mut []).unwrap();
    assert_eq!(state.snapshot(), before);
}
#[test]
fn reset_restores_the_initial_state() {
    let mut state = WriteState::new(8);
    state.claim(5);
    state.claim(5);
    assert!(state.snapshot().overflow);
    state.reset();
    assert_eq!(state.snapshot(), WriteState::new(8).snapshot());
    assert_eq!(state.logged.load(Ordering::Relaxed), 0);
    assert_eq!(state.claim(8), Claim::Ok { offset : 0 });
}
#[test]
fn valid_len_is_the_minimum_of_the_three_values() {
    let cases = [
        (10, 20, 30, 10),
        (20, 10, 30, 10),
        (20, 30, 10, 10),
        (NO_OVERFLOW, 7, 9, 7),
        (NO_OVERFLOW, 12, 9, 9),
        (0, 12, 9, 0),
    ];
    for (overflow_min, ptr, capacity, expected) in cases {
        let s = StateSnapshot {
            ptr,
            overflow_min,
            capacity,
            overflow: overflow_min != NO_OVERFLOW,
        };
        assert_eq!(s.valid_len(), expected);
    }
}
fn run_threads(capacity: u64, threads: usize, per_thread: usize, group: bool) {
    let state = WriteState::new(capacity);
    let claimed: Vec<Vec<(u64, u64)>> = thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let state = &state;
                scope
                    .spawn(move || {
                        let mut won = Vec::new();
                        let mut i = 0usize;
                        while i < per_thread {
                            if group {
                                let lens: Vec<u64> = (0..4)
                                    .map(|k| ((t + i + k) % 7 + 1) as u64)
                                    .collect();
                                let mut out = vec![0u64; 4];
                                state.claim_group(&lens, &mut out).unwrap();
                                for (len, offset) in lens.iter().zip(&out) {
                                    if *offset != INVALID_OFFSET {
                                        won.push((*offset, *len));
                                    }
                                }
                                i += 4;
                            } else {
                                let len = ((t + i) % 7 + 1) as u64;
                                if let Claim::Ok { offset } = state.claim(len) {
                                    won.push((offset, len));
                                }
                                i += 1;
                            }
                        }
                        won
                    })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut all: Vec<(u64, u64)> = claimed.into_iter().flatten().collect();
    all.sort();
    let mut pos = 0u64;
    for (offset, len) in &all {
        assert_eq!(* offset, pos, "intervals must be disjoint and gapless");
        pos += len;
    }
    let s = state.snapshot();
    assert_eq!(pos, s.valid_len());
    assert!(pos <= capacity);
    if s.overflow {
        assert_eq!(s.overflow_min, pos);
    }
}
#[test]
fn parallel_single_claims_are_disjoint() {
    run_threads(1 << 20, 8, 2000, false);
    run_threads(5_000, 8, 2000, false);
}
#[test]
fn parallel_group_claims_are_disjoint() {
    run_threads(1 << 20, 8, 2000, true);
    run_threads(5_000, 8, 2000, true);
}
#[test]
fn only_one_caller_wins_the_log_slot() {
    let state = WriteState::new(77);
    let wins: usize = thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| (0..100).filter(|_| state.log_once(1, Some(2))).count())
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    assert_eq!(wins, 1);
}
#[test]
fn overflow_is_logged_exactly_once_per_state() {
    const CAPACITY: u64 = 7_770_001;
    let installed = capture_installed();
    let region = WritePointerRegion::new(handle(CAPACITY));
    thread::scope(|scope| {
        for _ in 0..4 {
            scope
                .spawn(|| {
                    for _ in 0..50 {
                        region.state().claim(CAPACITY);
                    }
                });
        }
    });
    let mut out = [0u64; 2];
    region.state().claim_group(&[1, 2], &mut out).unwrap();
    region.result();
    region.result();
    assert_eq!(region.state().logged.load(Ordering::Relaxed), 1);
    if installed {
        assert_eq!(captured(CAPACITY), 1);
    }
}
#[test]
fn result_logs_a_device_reported_overflow_once() {
    const CAPACITY: u64 = 7_770_002;
    let installed = capture_installed();
    let region = WritePointerRegion::new(handle(CAPACITY));
    region.result();
    assert_eq!(region.state().logged.load(Ordering::Relaxed), 0);
    region.state().overflow_flag.store(1, Ordering::Release);
    region.state().overflow_min.store(40, Ordering::Release);
    region.state().ptr.store(CAPACITY + 9, Ordering::Release);
    let first = region.result();
    let second = region.result();
    assert_eq!(first, second);
    assert_eq!(region.state().logged.load(Ordering::Relaxed), 1);
    if installed {
        assert_eq!(captured(CAPACITY), 1);
    }
}
#[test]
fn no_log_without_overflow() {
    const CAPACITY: u64 = 7_770_003;
    let installed = capture_installed();
    let region = WritePointerRegion::new(handle(CAPACITY));
    region.state().claim(CAPACITY);
    region.result();
    assert_eq!(region.state().logged.load(Ordering::Relaxed), 0);
    if installed {
        assert_eq!(captured(CAPACITY), 0);
    }
}
#[test]
fn from_snapshot_builds_complete_and_partial_results() {
    let region = handle(100);
    let ok = StateSnapshot {
        ptr: 60,
        overflow_min: NO_OVERFLOW,
        capacity: 100,
        overflow: false,
    };
    let complete = WriteResult::from_snapshot(region, &ok);
    assert_eq!(complete, WriteResult::Complete { region, len : 60 });
    assert_eq!(complete.valid_len(), 60);
    let bad = StateSnapshot {
        ptr: 130,
        overflow_min: 90,
        capacity: 100,
        overflow: true,
    };
    let partial = WriteResult::from_snapshot(region, &bad);
    assert_eq!(
        partial, WriteResult::Partial { region, valid_len : 90, overflow_pos : 90 }
    );
    assert_eq!(partial.valid_len(), 90);
}
#[test]
fn region_wraps_the_handle_unchanged_and_uses_its_length_as_capacity() {
    let h = handle(256);
    let region = WritePointerRegion::new(h);
    assert_eq!(region.region(), h);
    assert_eq!(region.region().generation(), Generation(3));
    assert_eq!(region.state().snapshot().capacity, 256);
    assert_eq!(region.result(), WriteResult::Complete { region : h, len : 0 });
    assert_eq!(region.state().claim(200), Claim::Ok { offset : 0 });
    assert_eq!(region.state().claim(100), Claim::Invalid);
    assert_eq!(
        region.result(), WriteResult::Partial { region : h, valid_len : 200, overflow_pos
        : 200 }
    );
}
#[test]
fn device_source_has_no_shared_memory_and_one_pointer_atomic_per_warp() {
    assert!(! WRITE_POINTER_DEVICE_SRC.contains("extern __shared__"));
    assert!(! WRITE_POINTER_DEVICE_SRC.contains("__shared__"));
    for token in [
        "struct WriteState",
        "wp_claim(",
        "wp_claim_warp(",
        "__activemask",
        "__shfl_up_sync",
        "atomicMin",
        "sizeof(WriteState) == 32",
    ] {
        assert!(WRITE_POINTER_DEVICE_SRC.contains(token), "missing {token}");
    }
    let warp = WRITE_POINTER_DEVICE_SRC
        .split("unsigned long long wp_claim_warp(")
        .nth(1)
        .unwrap();
    assert_eq!(warp.matches("atomicAdd").count(), 1);
    assert_eq!(warp.matches("atomicAdd(&s->ptr").count(), 1);
}
const HARNESS: &str = r#"
#include <cstdio>
#include <cstdlib>
#include <vector>
#include <algorithm>
#include <utility>
typedef unsigned long long ull;

__global__ void k_claim(WriteState* s, const ull* lens, ull* out, int n, int mode) {
  int i = blockIdx.x * blockDim.x + threadIdx.x;
  if (i >= n) return;
  const ull len = lens[i];
  ull r;
  if (mode == 0) {
    r = wp_claim(s, len);
  } else if (mode == 1) {
    r = wp_claim_warp(s, len);
  } else if ((threadIdx.x % 3u) != 0u) {
    r = wp_claim_warp(s, len);
  } else {
    r = wp_claim(s, len);
  }
  out[i] = r;
}

static int fail(const char* what, int mode, int scenario) {
  printf("FAIL %s (mode %d, scenario %d)\n", what, mode, scenario);
  return 1;
}

static int run(int mode, int scenario, int n) {
  const bool zeros = scenario < 2;
  ull* lens;
  ull* out;
  WriteState* s;
  cudaMallocManaged(&s, sizeof(WriteState));
  cudaMallocManaged(&lens, n * sizeof(ull));
  cudaMallocManaged(&out, n * sizeof(ull));
  ull sum = 0;
  for (int i = 0; i < n; i++) {
    ull v = (ull)(((unsigned)i * 2654435761u) >> 7) % 9ull;
    if (!zeros && v == 0) v = 1;
    lens[i] = v;
    sum += v;
  }
  ull cap = sum + 1000;
  if (scenario == 1) cap = sum;
  if (scenario == 2) cap = sum / 2;
  if (scenario == 3) cap = 7;
  s->ptr = 0;
  s->overflow_min = 0xffffffffffffffffull;
  s->capacity = cap;
  s->overflow_flag = 0;
  s->logged = 0;
  k_claim<<<(n + 127) / 128, 128>>>(s, lens, out, n, mode);
  cudaError_t e = cudaDeviceSynchronize();
  if (e != cudaSuccess) {
    printf("FAIL cuda: %s\n", cudaGetErrorString(e));
    return 1;
  }
  int rc = 0;
  const bool expect_overflow = sum > cap;
  if (s->ptr != sum) rc = fail("ptr", mode, scenario);
  if ((s->overflow_flag != 0) != expect_overflow) rc = fail("flag", mode, scenario);
  if ((s->logged != 0) != expect_overflow) rc = fail("logged", mode, scenario);
  ull valid = std::min(std::min(s->overflow_min, s->ptr), s->capacity);
  std::vector<std::pair<ull, ull> > won;
  int failed = 0;
  for (int i = 0; i < n; i++) {
    if (out[i] == 0xffffffffffffffffull) {
      failed++;
    } else {
      if (out[i] + lens[i] > cap) rc = fail("range", mode, scenario);
      if (lens[i] > 0) won.push_back(std::make_pair(out[i], lens[i]));
    }
  }
  if ((failed > 0) != expect_overflow) rc = fail("failed count", mode, scenario);
  std::sort(won.begin(), won.end());
  ull pos = 0;
  for (size_t k = 0; k < won.size(); k++) {
    if (won[k].first != pos) { rc = fail("tiling", mode, scenario); break; }
    pos += won[k].second;
  }
  if (pos != valid) rc = fail("valid length", mode, scenario);
  cudaFree(s);
  cudaFree(lens);
  cudaFree(out);
  return rc;
}

int main() {
  int devices = 0;
  if (cudaGetDeviceCount(&devices) != cudaSuccess || devices == 0) {
    printf("SKIP\n");
    return 0;
  }
  int rc = 0;
  for (int mode = 0; mode < 3; mode++) {
    for (int scenario = 0; scenario < 4; scenario++) {
      rc |= run(mode, scenario, 4096 + 37);
    }
  }
  if (rc == 0) printf("OK\n");
  return rc;
}
"#;
fn find_nvcc() -> Option<std::path::PathBuf> {
    let fixed = std::path::PathBuf::from("/usr/local/cuda-13.3/bin/nvcc");
    if fixed.exists() {
        return Some(fixed);
    }
    std::process::Command::new("nvcc")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|_| std::path::PathBuf::from("nvcc"))
}
#[test]
fn device_source_compiles_and_claims_correctly_on_gpu() {
    let Some(nvcc) = find_nvcc() else {
        return;
    };
    let dir = std::env::temp_dir()
        .join(format!("xy_write_pointer_nvcc_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("harness.cu");
    let exe = dir.join("harness");
    std::fs::write(&src, format!("{WRITE_POINTER_DEVICE_SRC}\n{HARNESS}")).unwrap();
    let build = std::process::Command::new(&nvcc)
        .arg("-arch=sm_75")
        .arg("-o")
        .arg(&exe)
        .arg(&src)
        .output()
        .unwrap();
    assert!(
        build.status.success(), "nvcc failed: {}", String::from_utf8_lossy(& build
        .stderr)
    );
    let run = std::process::Command::new(&exe).output().unwrap();
    let stdout = String::from_utf8_lossy(&run.stdout).into_owned();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(run.status.success(), "harness failed: {stdout}");
    let line = stdout.trim();
    assert!(line == "OK" || line == "SKIP", "unexpected harness output: {stdout}");
}
