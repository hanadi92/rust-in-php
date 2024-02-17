#[cfg(test)]
mod tests {
    #![feature(test)]
    extern crate test;

    use std::ffi::{c_char, CString};
    use test::Bencher;
    use mytrain::find_departing_trains;

    #[bench]
    fn bench_workload(b: &mut Bencher) {
        let c_time_str = CString::new("14:54:20").unwrap();
        let c_time: *const c_char = c_time_str.as_ptr() as *const c_char;
        let c_place_str = CString::new("Tokoyo").unwrap();
        let c_place: *const c_char = c_place_str.as_ptr() as *const c_char;

        b.iter(|| find_departing_trains(c_time, c_place));
    }
}

fn main() {}