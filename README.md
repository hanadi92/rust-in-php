# Rust in PHP

Hypothesis: executing Rust code for high load tasks in PHP projects would improve performance.

Use case: the high load task is to find the trains the depart from a given location on a given time (within 10 minutes).

Strategy: building two implementations, in PHP and Rust. Measure performance and find hypothesis results.

Dataset: train schedule in Stockholm of 2500~ records. Taken from [Kaggle](https://www.kaggle.com/datasets/abdeaitali/commuter-train-timetable).

### ffi solution
``cd _ffi``

Build rust with ``cargo build --release --target-dir target``

Run php with ``php ffi.php``

Benchmark the rust implementation with ``cargo bench``

### extension solution

``cd _extension``
``cargo build --release --target-dir target``
``php -d extension=target/release/libextension.dylib extension.php``
