// The Eq trait is for equality comparisons that are equivalence relations. It requires a == b and
// a!= b are strict inverses, as well as the reflexive (a == a), symmetric (a == b => b == a) and
// transitive (a == b and b == c => a == c) properties
use std::cmp::Eq;
// Since the properties of Eq cannot be checked by the compiler, Eq implies PartialEq, and the
// trait itself is empty
use std::cmp::PartialEq;

// TODO make a list of these structs indexable
#[derive(Debug)]
pub struct MultilevelListItem {
    pub content: String,
    pub is_folder: bool,
    // TODO items should be an option type with None for non-folders and Some(MultilevelListItem)
    // for folders
    pub items: Vec<MultilevelListItem>,
    // TODO add a field called hash that keeps track of the hash of the items for folders so that
    // our comparison times aren't O(n) where n is the total number of nodes for this tree
}

impl MultilevelListItem {
    // Associated function to return an empty list
    pub fn new() -> MultilevelListItem {
        MultilevelListItem {
            content: String::from("root"),
            is_folder: false,
            items: Vec::new(),
        }
    }

    // This function intentionally takes ownership of the string that is added to the list
    // TODO this should return a Result type that contains any errors that might have occurred, or
    // nothing
    pub fn add_item(&mut self, item: &str, is_folder: bool) {
        let new_item = MultilevelListItem {
            content: item.clone().to_string(),
            is_folder,
            items: Vec::new(),
        };

        if !self.is_folder {
            self.is_folder = true;
        }

        self.items.push(new_item);
    }
}

impl PartialEq for MultilevelListItem {
    fn eq(&self, other: &Self) -> bool {
        // If neither item being compared is a folder, then just compare the values that they
        // contain
        if !self.is_folder && !other.is_folder {
            return self.content == other.content
        }

        // If one of them is a folder but the other isn't, they can't be equal
        if self.is_folder != other.is_folder {
            return false;
        }

        // Both of the items are folders
        // So we perform a sanity check for the number of items in both having to match
        if self.items.len() != other.items.len() {
            return false;
        }

        // Both folders have the same number of items, so we must check all the items
        for i in 0..self.items.len() {
            // It may not look like it, but this function is recursive. Comparison via the == or !=
            // operations happen by calling the `eq` function
            if self.items[i] != other.items[i] {
                return false;
            }
        }

        // If we've reached this far, all the items in both folders are equal
        // And hence both of these items are also equal
        return true;
    }
}

impl Eq for MultilevelListItem {}

#[cfg(test)]
mod tests {
    use crate::multilevel_list::*;

    #[test]
    fn create_new_list() {
        let new_list = MultilevelListItem::new();

        assert_eq!(new_list.content, "root");
        assert_eq!(new_list.is_folder, false);
        // assert_eq!(new_list.items, Vec::<MultilevelListItem>::new());
    }

    #[test]
    fn add_to_list() {
        let content1 = "Hello, world";
        let content2 = "Buy milk!";
        let mut list = MultilevelListItem::new();
        list.add_item(&content1, false);
        list.add_item(&content2, false);

        println!("{:?}", list);

        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].content, content1);
        assert_eq!(list.items[1].content, content2);
    }

    #[test]
    fn add_folder() {
        let content1 = "Hello, world";
        let mut list = MultilevelListItem::new();
        list.add_item(&content1, false);

        // Create a new folder "bookmarks"
        list.add_item("bookmarks", true);

        // Add item to folder
        // We don't want to take ownership of the item in the list
        let folder = &mut list.items[1];

        let content2 = "Buy milk!";
        folder.add_item(&content2, false);

        println!("{:?}", list);

        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0].content, content1);
        assert_eq!(list.items[1].content, "bookmarks");
        assert_eq!(list.items[1].is_folder, true);
        assert_eq!(list.items[1].items[0].content, content2);
    }

    #[test]
    fn test_equivalence() {
        let content1 = "Hello";
        let content2 = "World";

        let mut list1 = MultilevelListItem::new();
        let mut list2 = MultilevelListItem::new();

        // We're adding the same item to both lists so that they'll be equal
        list1.add_item(&content1, false);
        list2.add_item(&content1, false);

        assert_eq!(list1, list2);
        // Sanity check, cuz I'm paranoid
        assert_eq!(list1 == list2, true);

        // Now we add another item to list1 so that the lists are no longer equal
        list1.add_item(&content2, false);
        
        assert_ne!(list1, list2);
        // Sanity check, cuz I'm paranoid
        assert_eq!(list1 != list2, true);

        // We make the lists equal again
        list2.add_item(&content2, false);

        assert_eq!(list1, list2);

        // Now it's time to add a folder with content

        let folder_name = "bookmarks";
        // Create a new folder "bookmarks"
        list1.add_item(folder_name, true);

        // Add item to folder
        // We don't want to take ownership of the item in the list
        let folder = &mut list1.items[2];

        let content = "Buy milk!";
        folder.add_item(&content, false);

        // Obviously the lists will be different now
        assert_ne!(list1, list2);
        // Sanity check, cuz I'm paranoid
        assert_eq!(list1 != list2, true);

        // And now we do the same for list2
        // Create a new folder "bookmarks"
        list2.add_item(folder_name, true);

        // Add item to folder
        // We don't want to take ownership of the item in the list
        let folder = &mut list2.items[2];

        let content = "Buy milk!";
        folder.add_item(&content, false);

        println!("{:?}", list1);

        // And they should be equal again!
        assert_eq!(list1, list2);
        // Sanity check, cuz I'm paranoid
        assert_eq!(list1 == list2, true);
    }
}
