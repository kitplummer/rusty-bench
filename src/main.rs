use tiny_bench::black_box;

fn main() {
    // Results are compared by label
    let label = "compare_functions";
    tiny_bench::bench_labeled(label, my_slow_function);
    tiny_bench::bench_labeled(label, my_faster_function);
    // prints:
    //compare_functions [30.3 thousand iterations in 5.24s with 100.0 samples]:
    //elapsed	[min mean max]:	[246.33µs 175.51µs 246.33µs]
    //compare_functions [60.6 thousand iterations in 5.24s with 100.0 samples]:
    //elapsed	[min mean max]:	[87.67µs 86.42µs 87.67µs]
    //change	[min mean max]:	[-49.6111% -50.7620% -64.4102%] (p = 0.00)
}

fn my_slow_function() {
    let mut num_iters = 0;
    for _ in 0..10_000 {
        num_iters += black_box(1);
    }
    assert_eq!(10_000, black_box(num_iters))
}

fn my_faster_function() {
    let mut num_iters = 0;
    for _ in 0..5_000 {
        num_iters += black_box(1);
    }
    assert_eq!(5_000, black_box(num_iters))
}
