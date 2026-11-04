use super::*;
use memmap2::MmapMut;
fn anon(len: usize) -> MmapAdvice {
    MmapAdvice::new(MmapMut::map_anon(len).unwrap().make_read_only().unwrap())
}
#[test]
fn all_hints_accepted_on_valid_range() {
    let a = anon(3 * 4096);
    for hint in [Hint::WillNeed, Hint::Sequential, Hint::Random, Hint::Cold] {
        a.advise(100..9000, hint).unwrap();
    }
}
#[test]
fn out_of_bounds_and_empty_ranges() {
    let a = anon(4096);
    assert!(a.advise(0..4097, Hint::WillNeed).is_err());
    a.advise(10..10, Hint::Cold).unwrap();
}
#[cfg(target_os = "linux")]
mod residency {
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::io::AsRawFd;
    use std::time::{Duration, Instant};
    use super::*;
    const PAGES: usize = 64;
    fn resident_pages(map: &memmap2::Mmap) -> usize {
        let mut vec = vec![0u8; PAGES];
        let rc = unsafe {
            libc::mincore(
                map.as_ptr() as *mut libc::c_void,
                PAGES * 4096,
                vec.as_mut_ptr(),
            )
        };
        assert_eq!(rc, 0);
        vec.iter().filter(|b| **b & 1 == 1).count()
    }
    fn evicted_file(tag: &str) -> (std::path::PathBuf, memmap2::Mmap) {
        let path = std::env::temp_dir()
            .join(format!("xy-advice-{tag}-{}", std::process::id()));
        let mut file = File::create(&path).unwrap();
        file.write_all(&vec![7u8; PAGES * 4096]).unwrap();
        file.sync_all().unwrap();
        let file = File::open(&path).unwrap();
        let rc = unsafe {
            libc::posix_fadvise(file.as_raw_fd(), 0, 0, libc::POSIX_FADV_DONTNEED)
        };
        assert_eq!(rc, 0);
        let map = unsafe { memmap2::Mmap::map(&file) }.unwrap();
        (path, map)
    }
    #[test]
    fn will_need_brings_pages_into_the_page_cache() {
        let (path, map) = evicted_file("willneed");
        let before = resident_pages(&map);
        let advice = MmapAdvice::new(map);
        advice.advise(0..PAGES * 4096, Hint::WillNeed).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut after = 0;
        while Instant::now() < deadline {
            after = resident_pages(&advice.map);
            if after > before {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        std::fs::remove_file(path).ok();
        assert!(after > before, "resident pages {before} -> {after}");
    }
    #[test]
    fn cold_keeps_contents_and_does_not_drop_pages() {
        let (path, map) = evicted_file("cold");
        let advice = MmapAdvice::new(map);
        let sum: u64 = advice.map.iter().map(|b| *b as u64).sum();
        let before = resident_pages(&advice.map);
        advice.advise(0..PAGES * 4096, Hint::Cold).unwrap();
        let after_sum: u64 = advice.map.iter().map(|b| *b as u64).sum();
        let after = resident_pages(&advice.map);
        std::fs::remove_file(path).ok();
        assert_eq!(sum, after_sum);
        assert!(before > 0 && after >= before / 2, "{before} -> {after}");
    }
}
