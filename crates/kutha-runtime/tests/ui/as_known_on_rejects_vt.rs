use kutha_common::ValidTime;
use kutha_runtime::Runtime;

fn main() {
    let rt = Runtime::default();
    let _ = rt.as_known_on(ValidTime::from(2017));
}
