pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    for i in 1..=n{
        if n%i==0{
            match i {
                3 => res.push_str("Pling"),
                5 => res.push_str("Plang"),
                7 => res.push_str("Plong"),
                _ => {}
            }
        }
    }
    if res.len()==0{
        res = format!("{n}");
    }
    println!("{res}");
    res
}


fn main() {
    assert_eq!(raindrops(105), "PlingPlangPlong");
}
