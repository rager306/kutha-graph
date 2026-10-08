use kutha_runtime::Runtime;

fn main() {
    let rt = Runtime::default();
    let _ = rt.as_of(2017u64);
}
