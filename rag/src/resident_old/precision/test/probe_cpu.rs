use super::*;
use crate::resident::precision::AxisId;

fn probe(features: CpuFeatures, cores: usize) -> CapabilitySet {
    CpuProbe::new(Box::new(FixedCpu { features, logical_cores: cores }), ProbeConfig::default())
        .probe()
        .unwrap()
}

fn avx2() -> CpuFeatures {
    CpuFeatures { avx2: true, f16c: true, fma: true, vnni: false }
}

#[test]
fn avx2_host_offers_f16c_and_int8_without_vnni() {
    let set = probe(avx2(), 4);
    assert!(set.contains(&OPERANDS, &FP16_TO_FP32));
    assert!(set.contains(&SCORING, &INT8));
    assert!(!set.contains(&SCORING, &INT8_VNNI));
}

#[test]
fn vnni_host_additionally_lists_vnni_path() {
    let set = probe(CpuFeatures { vnni: true, ..avx2() }, 4);
    assert!(set.contains(&SCORING, &INT8));
    assert!(set.contains(&SCORING, &INT8_VNNI));
}

#[test]
fn host_without_simd_features_keeps_only_fp32() {
    let set = probe(CpuFeatures::default(), 1);
    assert!(set.contains(&OPERANDS, &FP32));
    assert!(!set.contains(&OPERANDS, &FP16_TO_FP32));
    assert!(!set.contains(&SCORING, &INT8));
}

#[test]
fn thread_counts_reach_the_logical_core_count() {
    let set = probe(avx2(), 6);
    let threads: Vec<_> = set
        .values(&AxisId::new("threads"))
        .unwrap()
        .iter()
        .map(|v| v.as_str().parse::<u32>().unwrap())
        .collect();
    assert_eq!(threads, [1, 2, 4, 6]);
}

#[test]
fn single_core_has_one_thread_value() {
    let set = probe(avx2(), 1);
    assert_eq!(set.values(&THREADS).unwrap().len(), 1);
}

#[test]
fn host_source_reports_at_least_one_core() {
    assert!(HostCpu.logical_cores() >= 1);
}
