mod levenshtein;

mod multilevel_list;

fn main() {
    println!("Hello, world!");
    let s1 = "NOHELLO";
    let s2 = "HELLGO";
    println!("Edit distance between {} and {} is {}", s1, s2, levenshtein::levenshtein_tabulation(&s1.as_bytes(), &s2.as_bytes()));

    let mut list1 = multilevel_list::MultilevelListItem::new();
    let mut list2 = multilevel_list::MultilevelListItem::new();

    let content1 = "Hello";

    // We're adding the same item to both lists so that they'll be equal
    list1.add_file(content1);
    list2.add_file(content1);

    let dist = levenshtein::levenshtein_tabulation(&list1.items().unwrap(), &list2.items().unwrap());
    println!("Edit distance: {}", dist);
    assert_eq!(dist, 0);
    

    list1.add_file("Hey");

    let dist = levenshtein::levenshtein_tabulation(&list1.items().unwrap(), &list2.items().unwrap());
    println!("Edit distance: {}", dist);
    assert_eq!(dist, 1);
}
