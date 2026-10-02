use kutha_common::{TransactionTime, ValidTime, WallTime};

fn takes_valid_time(_vt: ValidTime) {}

fn main() {
    takes_valid_time(TransactionTime::from(0));
    takes_valid_time(WallTime::from(0));
}
