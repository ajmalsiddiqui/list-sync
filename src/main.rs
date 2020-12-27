mod levenshtein;

fn main() {
    println!("Hello, world!");
    let s1 = "NOHELLO";
    let s2 = "HELLGO";
    println!("Edit distance between {} and {} is {}", s1, s2, levenshtein::levenshtein_tabulation(&s1.as_bytes(), &s2.as_bytes()));
}
