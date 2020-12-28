// The Eq trait is for equality comparisons that are equivalence relations. It requires a == b and
// a!= b are strict inverses, as well as the reflexive (a == a), symmetric (a == b => b == a) and
// transitive (a == b and b == c => a == c) properties
use std::cmp::Eq;
// Since the properties of Eq cannot be checked by the compiler, Eq implies PartialEq, and the
// trait itself is empty
use std::cmp::PartialEq;
use crate::multilevel_list::MultilevelListItem::*;

// TODO make a list of these structs indexable
#[derive(Debug)]
pub enum MultilevelListItem {
    File(String),
    Folder(String, Vec<MultilevelListItem>)
}

impl MultilevelListItem {
    fn add_item(&mut self, item: MultilevelListItem) {
        if let Folder(_, me) = self {
            me.push(item)
        }
    }

    pub fn new() -> MultilevelListItem {
        Folder("root".to_string(), Vec::new())
    }

    pub fn add_folder(&mut self, name: &str) {
        self.add_item(Folder(name.to_string(), Vec::new()))
    }

    pub fn add_file(&mut self, content: &str) {
        self.add_item(File(content.to_string()))
    }
    
    pub fn items(&mut self) -> Option<&mut Vec<MultilevelListItem>> {
        if let Folder(_, me) = self { Some(me) } else { None }
    }

    pub fn is_file(&self) -> bool {
        matches!(*self, File(_))
    }

    pub fn is_folder(&self) -> bool {
        !self.is_file()
    }
}

impl PartialEq for MultilevelListItem {

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (File(a), File(b)) => a == b,
            (Folder(n_a, a), Folder(n_b, b)) => {

                // Both of the items are folders
                // So we perform a sanity check for the number of items in both having to match
                if n_a != n_b {
                    return false
                } else if a.len() != b.len() {
                    false
                } else {
                    // Both folders have the same number of items, so we must check all the items
                    for i in 0..a.len() {
                        // It may not look like it, but this function is recursive. Comparison via the == or !=
                        // operations happen by calling the `eq` function
                        if a[i] != b[i] {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false
        }
    }
}

impl Eq for MultilevelListItem {}

#[cfg(test)]
mod tests {
    use crate::multilevel_list::*;

    fn verify_file_content(file: &MultilevelListItem, content: &str) {
        assert!(file.is_file());
        if let File(t_file) = file {
            assert_eq!(t_file, content);
        }
    }

    fn verify_folder(folder: &MultilevelListItem, name: &str) {
        assert!(folder.is_folder());
        if let Folder(f_name, _) = folder {
            assert_eq!(f_name, name);
        }
    }

    #[test]
    fn create_new_list() {
        let new_list = MultilevelListItem::new();

        assert!(new_list.is_folder());
        if let Folder(name, t_list) = new_list {
            assert_eq!(t_list.len(), 0);
            assert_eq!(name, "root");
        }
    }

    #[test]
    fn add_to_list() {
        let content1 = "Hello, world";
        let content2 = "Buy milk!";
        let mut list = MultilevelListItem::new();
        list.add_file(content1);
        list.add_file(content2);

        println!("{:?}", list);

        let items = list.items().unwrap();
        assert_eq!(items.len(), 2);

        verify_file_content(&items[0], content1);
        verify_file_content(&items[1], content2);
    }

    #[test]
    fn add_folder() {
        let content1 = "Hello, world";
        let mut list = MultilevelListItem::new();
        list.add_file(content1);

        // Create a new folder "bookmarks"
        list.add_folder("bookmarks");

        // Add item to folder
        // We don't want to take ownership of the item in the list
        let folder = &mut list.items().unwrap()[1];

        let content2 = "Buy milk!";
        folder.add_file(content2);

        println!("{:?}", list);

        let items = list.items().unwrap();
        assert_eq!(items.len(), 2);
        verify_file_content(&items[0], content1);

        verify_folder(&items[1], "bookmarks");
        verify_file_content(&items[1].items().unwrap()[0], content2);
    }

    #[test]
    fn test_equivalence() {
        let content1 = "Hello";
        let content2 = "World";

        let mut list1 = MultilevelListItem::new();
        let mut list2 = MultilevelListItem::new();

        // We're adding the same item to both lists so that they'll be equal
        list1.add_file(content1);
        list2.add_file(content1);

        assert_eq!(list1, list2);

        // Now we add another item to list1 so that the lists are no longer equal
        list1.add_file(content2);
        
        assert_ne!(list1, list2);

        // We make the lists equal again
        list2.add_file(content2);

        assert_eq!(list1, list2);

        // Now it's time to add a folder with content

        let folder_name = "bookmarks";
        // Create a new folder "bookmarks"
        list1.add_folder(folder_name);

        // Add item to folder
        // We don't want to take ownership of the item in the list
        let folder = &mut list1.items().unwrap()[2];

        let content = "Buy milk!";
        folder.add_file(content);

        // Obviously the lists will be different now
        assert_ne!(list1, list2);

        // And now we do the same for list2
        // Create a new folder "bookmarks"
        list2.add_folder(folder_name);

        // Add item to folder
        // We don't want to take ownership of the item in the list
        let folder = &mut list2.items().unwrap()[2];

        let content = "Buy milk!";
        folder.add_file(content);

        println!("{:?}", list1);

        // And they should be equal again!
        assert_eq!(list1, list2);
        // Sanity check, cuz I'm paranoid
        assert_eq!(list1 == list2, true);
    }
}
