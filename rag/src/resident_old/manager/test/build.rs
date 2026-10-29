use super::*;
use crate::resident::index::memory::InMemoryIndex;
use crate::resident::kernel::mock::MockKernel;
use crate::resident::memory::{ArenaKind, DeviceKind};
use crate::resident::memory::ArenaAllocator;
use std::sync::Arc;
struct Zeros;
impl RangeSource for Zeros {
    fn read(&self, range: WrittenRange) -> Vec<u8> {
        vec![0; range.len as usize]
    }
}
fn id(n: u64) -> EntryId {
    EntryId::new(n)
}
fn host_arena(cap: u64) -> ArenaAllocator {
    ArenaAllocator::new(ArenaKind::IndexSegment, DeviceKind::Cpu, cap)
}
fn span(offset: u64, len: u64) -> RegionSpan {
    RegionSpan { offset, len }
}
#[test]
fn claims_beyond_the_cap_set_overflow_and_keep_the_prefix_valid() {
    let c = WriteCursor::new(1000, 100);
    assert_eq!(c.claim(40), Some(1000..1040));
    assert_eq!(c.claim(40), Some(1040..1080));
    assert!(! c.overflowed());
    assert_eq!(c.claim(40), None);
    assert!(c.overflowed());
    assert_eq!(c.claim(1), None, "later claims fail even when they would fit");
    assert_eq!(c.valid_len(), 80);
}
#[test]
fn group_claim_matches_sequential_claims() {
    let sizes = [10, 20, 30, 40, 50];
    let grouped = WriteCursor::new(0, 100).claim_group(&sizes);
    let single = WriteCursor::new(0, 100);
    let sequential: Vec<_> = sizes.iter().map(|s| single.claim(*s)).collect();
    assert_eq!(grouped, sequential);
    assert_eq!(grouped.iter().flatten().count(), 4);
}
#[test]
fn concurrent_claims_are_disjoint_and_form_a_prefix() {
    let c = Arc::new(WriteCursor::new(64, 10_000));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = c.clone();
            std::thread::spawn(move || {
                (0..100).filter_map(|_| c.claim(16)).collect::<Vec<_>>()
            })
        })
        .collect();
    let mut got: Vec<_> = handles.into_iter().flat_map(|h| h.join().unwrap()).collect();
    got.sort_by_key(|r| r.start);
    assert_eq!(got.len(), 625);
    for (i, r) in got.iter().enumerate() {
        assert_eq!(r.start, 64 + i as u64 * 16);
    }
    assert!(c.overflowed());
    assert_eq!(c.valid_len(), 10_000);
}
#[test]
fn variable_table_resolves_at_boundaries() {
    let t = OffsetTable::new(500, &[(id(1), 10), (id(2), 0), (id(3), 25)]);
    assert_eq!(t.offsets(), & [500, 510, 510, 535]);
    assert_eq!(t.addr_to_entry(499), None);
    assert_eq!(t.addr_to_entry(500), Some(id(1)));
    assert_eq!(t.addr_to_entry(509), Some(id(1)));
    assert_eq!(t.addr_to_entry(510), Some(id(3)));
    assert_eq!(t.addr_to_entry(534), Some(id(3)));
    assert_eq!(t.addr_to_entry(535), None);
}
#[test]
fn prefix_sum_of_nothing_is_the_base() {
    assert_eq!(exclusive_prefix_sum(7, & []), vec![7]);
    assert_eq!(exclusive_prefix_sum(0, & [3, 4]), vec![0, 3, 7]);
}
#[test]
fn similar_sizes_plan_a_fixed_stride_and_skewed_sizes_a_table() {
    let near: Vec<_> = [(1, 100), (2, 98), (3, 100)].map(|(e, l)| (id(e), l)).to_vec();
    let layout = BuiltLayout::plan(1000, &near, 50);
    assert!(matches!(layout, BuiltLayout::Fixed { stride : 100, .. }));
    assert_eq!(layout.addr_to_entry(1000), Some(id(1)));
    assert_eq!(layout.addr_to_entry(1199), Some(id(2)));
    assert_eq!(layout.addr_to_entry(1299), Some(id(3)));
    assert_eq!(layout.addr_to_entry(1300), None);
    let skewed: Vec<_> = [(1, 10), (2, 100)].map(|(e, l)| (id(e), l)).to_vec();
    let layout = BuiltLayout::plan(0, &skewed, 50);
    assert!(matches!(layout, BuiltLayout::Variable(_)));
    assert_eq!(layout.addr_to_entry(10), Some(id(2)));
}
#[test]
fn built_layout_becomes_a_bound_slot_map() {
    let items: Vec<_> = [(1, 16), (2, 16)].map(|(e, l)| (id(e), l)).to_vec();
    let map = BuiltLayout::plan(256, &items, 0).into_slot_map();
    assert_eq!(map.index_of(272), Some(1));
    assert_eq!(map.entry(1).map(| e | e.entry), Some(id(2)));
}
#[test]
fn skip_resumes_inside_a_span() {
    let input = [span(0, 100), span(500, 100), span(900, 100)];
    assert_eq!(skip_bytes(& input, 0), input.to_vec());
    assert_eq!(skip_bytes(& input, 128), vec![span(528, 72), span(900, 100)]);
    assert_eq!(skip_bytes(& input, 300), vec![]);
}
#[test]
fn region_shortage_yields_partial_results_and_the_build_resumes() {
    let alloc = host_arena(1 << 16);
    let arena = IndexArena::new(&alloc, 0);
    let kernel = MockKernel::embedding(0);
    let input = [span(0, 100), span(1000, 100), span(2000, 100)];
    let out = drive_build(&kernel, &arena, &BatchDesc::default(), &input, 128, 8)
        .unwrap();
    assert_eq!(out.partials, 2);
    assert_eq!(out.regions.len(), 3);
    assert_eq!(out.written.iter().map(| w | w.len).sum::< u64 > (), 300);
    assert_eq!(out.written[0], WrittenRange { offset : 0, len : 100 });
    assert_eq!(out.written[1], WrittenRange { offset : 1000, len : 28 });
    assert_eq!(out.written[2], WrittenRange { offset : 1028, len : 72 });
}
#[test]
fn missing_follow_up_region_is_an_error_and_releases_the_grants() {
    let alloc = host_arena(1 << 16);
    let arena = IndexArena::new(&alloc, 0);
    let kernel = MockKernel::embedding(0);
    let input = [span(0, 300)];
    let err = drive_build(&kernel, &arena, &BatchDesc::default(), &input, 128, 1)
        .unwrap_err();
    assert!(matches!(err, ManagerError::ArenaExhausted { .. }));
    assert!(arena.reserve_writable(1 << 15).is_ok(), "grants were returned");
}
#[test]
fn kernel_failure_surfaces_and_the_batch_is_ended() {
    let alloc = host_arena(1 << 16);
    let arena = IndexArena::new(&alloc, 0);
    let kernel = MockKernel::embedding(0).fail_after(0);
    let err = drive_build(&kernel, &arena, &BatchDesc::default(), &[span(0, 8)], 64, 4)
        .unwrap_err();
    assert!(matches!(err, ManagerError::Kernel(_)));
    assert!(
        kernel.calls().iter().any(| c | matches!(c, crate
        ::resident::kernel::mock::Call::End(_)))
    );
}
#[test]
fn rings_work_independently() {
    let rings = BuildRings::new();
    rings.push_writes(&[WrittenRange { offset: 0, len: 8 }]);
    assert_eq!(rings.pending(BuildRing::Embedding), 1);
    assert_eq!(rings.pending(BuildRing::MaxSim), 0);
    rings
        .push_hits(
            &[KernelHit { offset: 0, score: 1.0 }, KernelHit { offset: 8, score: 0.5 }],
        );
    assert_eq!(rings.drain_hits().len(), 2);
    assert_eq!(
        rings.pending(BuildRing::Embedding), 1, "draining hits leaves writes alone"
    );
    assert_eq!(rings.drain_writes().len(), 1);
    assert!(rings.drain_writes().is_empty());
}
#[test]
fn hits_into_a_segment_written_in_the_same_cycle_resolve() {
    let index = InMemoryIndex::new();
    let items: Vec<_> = [(1, 16), (2, 16), (3, 16)].map(|(e, l)| (id(e), l)).to_vec();
    let mut map = AddressMap::new();
    assert!(map.add_segment(0, BuiltLayout::plan(1000, & items, 0).into_slot_map()));
    let (mut q, mut t) = (UpdateQueue::new(), ResidencyTracker::new());
    let writer = MockKernel::embedding(1 << 20);
    let b = writer.begin(&BatchDesc::default()).unwrap();
    writer.run(b, &[span(1000, 48)]).unwrap();
    let rings = BuildRings::new();
    rings.push_writes(&writer.end(b).unwrap());
    rings
        .push_hits(
            &[
                KernelHit {
                    offset: 1040,
                    score: 0.9,
                },
            ],
        );
    let out = run_build_slot(&rings, &Zeros, &index, &mut map, &mut q, &mut t);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].entry, id(3));
    assert_eq!(t.len(), 3);
}
#[cfg(all(feature = "cuda", not(loom)))]
mod device {
    use crate::resident::subengine::DeviceHandle;
    use cudarc::driver::{LaunchConfig, PushKernelArg};
    use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};
    #[test]
    #[ignore = "requires a CUDA device"]
    fn warp_aggregated_cursor_claims_a_contiguous_prefix() {
        let dev = DeviceHandle::cuda(0).unwrap();
        let ctx = dev.cuda_context().unwrap().clone();
        let stream = ctx.new_stream().unwrap();
        let ptx = compile_ptx_with_opts(
                super::super::device::CURSOR_APPEND,
                CompileOptions {
                    arch: Some("compute_75"),
                    ..Default::default()
                },
            )
            .unwrap();
        let f = ctx.load_module(ptx).unwrap().load_function("cursor_append").unwrap();
        let (n, cap) = (1000u32, 10_000u64);
        let sizes = stream.clone_htod(&vec![16u64; n as usize]).unwrap();
        let mut cursor = stream.clone_htod(&[0u64]).unwrap();
        let mut limit = stream.clone_htod(&[u64::MAX]).unwrap();
        let mut overflow = stream.clone_htod(&[0u32]).unwrap();
        let mut offsets = stream.alloc_zeros::<u64>(n as usize).unwrap();
        let mut b = stream.launch_builder(&f);
        b.arg(&mut cursor)
            .arg(&mut limit)
            .arg(&mut overflow)
            .arg(&sizes)
            .arg(&mut offsets);
        b.arg(&n).arg(&cap);
        unsafe { b.launch(LaunchConfig::for_num_elems(n)) }.unwrap();
        stream.synchronize().unwrap();
        let mut got: Vec<u64> = stream
            .clone_dtoh(&offsets)
            .unwrap()
            .into_iter()
            .filter(|o| *o != u64::MAX)
            .collect();
        got.sort_unstable();
        assert_eq!(got, (0..625u64).map(| i | i * 16).collect::< Vec < _ >> ());
        assert_eq!(stream.clone_dtoh(& overflow).unwrap(), [1]);
        assert_eq!(stream.clone_dtoh(& limit).unwrap(), [cap]);
    }
}
#[cfg(all(feature = "cuda", not(loom)))]
mod device_visibility {
    use crate::resident::subengine::DeviceHandle;
    use cudarc::driver::{LaunchConfig, PushKernelArg};
    use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};
    use std::sync::atomic::{AtomicU64, Ordering};
    const PUBLISH: &str = r#"
extern "C" __global__ void publish(unsigned long long* data, unsigned long long* flag,
                                  unsigned long long round) {
    data[threadIdx.x] = round * 1000ull + threadIdx.x;
    FENCE;
    __syncthreads();
    if (threadIdx.x == 0) {
        asm volatile("st.relaxed.sys.u64 [%0], %1;" :: "l"(flag), "l"(round) : "memory");
    }
    long long t = clock64();
    while (clock64() - t < 20000000ll) {}
}
"#;
    fn polled_rounds_see_all_data(fence: &str) -> u32 {
        let dev = DeviceHandle::cuda(0).unwrap();
        let ctx = dev.cuda_context().unwrap().clone();
        let stream = ctx.new_stream().unwrap();
        let src = PUBLISH.replace("FENCE", fence);
        let ptx = compile_ptx_with_opts(
                &src,
                CompileOptions {
                    arch: Some("compute_75"),
                    ..Default::default()
                },
            )
            .unwrap();
        let f = ctx.load_module(ptx).unwrap().load_function("publish").unwrap();
        let mut host = unsafe { ctx.alloc_pinned::<u64>(33) }.unwrap();
        let base = host.as_mut_ptr().unwrap();
        let flag = unsafe { AtomicU64::from_ptr(base.add(32)) };
        let (data_dev, flag_dev) = (base as u64, base as u64 + 32 * 8);
        let mut stale = 0;
        for round in 1..=200u64 {
            for i in 0..32 {
                unsafe { base.add(i).write_volatile(0) };
            }
            flag.store(0, Ordering::Release);
            let mut b = stream.launch_builder(&f);
            b.arg(&data_dev).arg(&flag_dev).arg(&round);
            unsafe {
                b.launch(LaunchConfig {
                    grid_dim: (1, 1, 1),
                    block_dim: (32, 1, 1),
                    shared_mem_bytes: 0,
                })
            }
                .unwrap();
            while flag.load(Ordering::Acquire) != round {
                std::hint::spin_loop();
            }
            let seen: Vec<u64> = (0..32)
                .map(|i| unsafe { base.add(i).read_volatile() })
                .collect();
            if seen != (0..32).map(|i| round * 1000 + i).collect::<Vec<_>>() {
                stale += 1;
            }
            stream.synchronize().unwrap();
        }
        stale
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn data_is_visible_to_a_polling_host_after_a_device_scope_fence() {
        let stale = polled_rounds_see_all_data("__threadfence()");
        eprintln!("device-scope fence: {stale}/200 rounds with stale data");
        assert_eq!(stale, 0);
    }
    #[test]
    #[ignore = "requires a CUDA device"]
    fn data_is_visible_to_a_polling_host_after_a_system_scope_fence() {
        let stale = polled_rounds_see_all_data("__threadfence_system()");
        eprintln!("system-scope fence: {stale}/200 rounds with stale data");
        assert_eq!(stale, 0);
    }
}
