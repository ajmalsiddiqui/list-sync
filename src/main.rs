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
    list1.add_item(&content1, false);
    list2.add_item(&content1, false);

    let dist = levenshtein::levenshtein_tabulation(&list1.items, &list2.items);
    println!("Edit distance: {}", dist);
    assert_eq!(dist, 0);

    list1.add_item(&"Hey", false);
    let dist = levenshtein::levenshtein_tabulation(&list1.items, &list2.items);
    println!("Edit distance: {}", dist);
    assert_eq!(dist, 1);
}
