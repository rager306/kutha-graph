use kutha_runtime::Runtime;

fn main() {
    let rt = Runtime::default();
    let _ = rt.as_known_on(2017u64);
}
