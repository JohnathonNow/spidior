use std::time::Instant;

fn main() {
    let s = "a".repeat(1000);
    let mut chars = s.chars();
    let t0 = Instant::now();
    for i in 0..1000 {
        let _ = s.chars().nth(i);
    }
    println!("chars().nth() took {:?}", t0.elapsed());

    let bytes = s.as_bytes();
    let t1 = Instant::now();
    for i in 0..1000 {
        let _ = bytes[i];
    }
    println!("as_bytes() took {:?}", t1.elapsed());
}
