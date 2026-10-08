use kutha_common::WallTime;
use kutha_runtime::Runtime;

fn main() {
    let rt = Runtime::default();
    let _ = rt.as_of(WallTime::from(2017));
}
