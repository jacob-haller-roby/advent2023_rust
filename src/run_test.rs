use std::time::Instant;

pub fn exec(func: impl Fn() -> u32) {
    let now = Instant::now();
    let result = func();
    let elapsed = now.elapsed();
    println!("Result is: {}", result);
    println!("Elapsed: {:.2?}", elapsed);
}
