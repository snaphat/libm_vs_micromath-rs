use std::time::Instant;

fn libm_test() -> f32 {
    let before = Instant::now();
    let mut a = 0f32;
    for _ in 0..0xFFFFFFFFu32 {
        a += libm::cosf(1.5f32);
    }
    let after =  before.elapsed();
    println!("libm Elapsed time: {:.2?}", after);
    a as f32
}

fn micromath_test() -> f32 {
    let before = Instant::now();
    let mut a = 0f32;
    for _ in 0..0xFFFFFFFFu32 {
        a += micromath::F32::cos(1.5f32.into());
    }
    let after =  before.elapsed();
    println!("micromath Elapsed time: {:.2?}", after);
    a as f32
}

fn main()
{
    libm_test();
    micromath_test();
}
