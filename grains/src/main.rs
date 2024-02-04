pub fn square(s: u32) -> u64 {
    match s {
        n if n>0 && n<65=> (2 as u64).pow(s - 1),
        _ =>panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum::<u64>()
}

fn main() {
    total();
}
