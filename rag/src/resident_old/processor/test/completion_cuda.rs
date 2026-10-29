use std::time::{Duration, Instant};

use cudarc::driver::CudaContext;
use tokio::sync::Notify;

use super::*;

fn stream() -> Option<Arc<CudaStream>> {
    let ctx = CudaContext::new(0)
        .inspect_err(|e| eprintln!("SKIPPED: no usable CUDA device: {e}"))
        .ok()?;
    ctx.new_stream().ok()
}

fn busy_work(stream: &Arc<CudaStream>) {
    let host = vec![1.0f32; 1 << 22];
    let dev = stream.clone_htod(&host).unwrap();
    let _ = stream.alloc_zeros::<f32>(dev.len()).unwrap();
}

fn reap_until_done(board: &LaneBoard, completion: &dyn Completion) -> Vec<(LaneId, LaneOutcome)> {
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut seen = vec![];
    while seen.is_empty() {
        assert!(Instant::now() < deadline, "lane did not complete");
        board.reap(completion, |l, o| seen.push((l, o)));
        std::thread::sleep(Duration::from_millis(1));
    }
    seen
}

#[test]
fn polled_completion_reports_success() {
    let Some(stream) = stream() else { return };
    let board = LaneBoard::new(1, Arc::new(Notify::new()));
    let completion = PolledCompletion::new(stream.clone(), 1);
    let lane = board.reserve().unwrap();
    busy_work(&stream);
    board.launch(lane, &completion);
    assert_eq!(reap_until_done(&board, &completion), vec![(lane, LaneOutcome::Success)]);
    assert!(board.is_idle(lane));
}

#[test]
fn host_callback_signals_lane_once() {
    let Some(stream) = stream() else { return };
    let board = Arc::new(LaneBoard::new(1, Arc::new(Notify::new())));
    let completion = HostFnCompletion::new(stream.clone(), board.clone());
    for _ in 0..20 {
        let lane = board.reserve().unwrap();
        busy_work(&stream);
        board.launch(lane, &completion);
        let seen = reap_until_done(&board, &completion);
        assert_eq!(seen, vec![(lane, LaneOutcome::Success)]);
        stream.synchronize().unwrap();
        assert_eq!(board.reap(&completion, |_, _| panic!("processed twice")), 0);
    }
}

#[test]
fn unrecorded_event_is_a_failed_lane() {
    let Some(stream) = stream() else { return };
    let completion = PolledCompletion::new(stream, 1);
    assert_eq!(completion.poll(LaneId(0)), PollState::Ready(LaneOutcome::Failed));
}
