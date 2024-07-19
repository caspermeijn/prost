#![no_main]

use libfuzzer_sys::fuzz_target;
use tests::test_parse_date;

fuzz_target!(|data: &[u8]| test_parse_date(data));
