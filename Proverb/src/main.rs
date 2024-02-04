pub fn build_proverb(list: &[&str]) -> String {
    let mut res = String::new();
    if list.len()>0{
        for i in 0..=list.len() - 1 {
            if list.get(i + 1).is_some() {
                res.push_str(&format!(
                    "For want of a {str1} the {str2} was lost.\n",
                str1 = list.get(i).unwrap(),
                str2 = list.get(i + 1).unwrap()
            ));
            }
            if i == list.len() - 1 {
                res.push_str(&format!(
                    "And all for the want of a {str}.",
                str = list.get(0).unwrap()
            ));
            }
        }
    }
    
    println!("{res}");
    res
}

fn main() {
    let list = vec![];
    build_proverb(&list);
}
