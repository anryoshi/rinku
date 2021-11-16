use std::fmt;

#[derive(Debug)]
pub struct Stats {
    linked_items : u32,
    links_created: u32
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            linked_items: 0,
            links_created: 0
        }
    }

    pub fn aggregate(&mut self, other : &Stats) {
        self.linked_items += other.linked_items;
        self.links_created += other.links_created;
    }

    pub fn new_item<'a>(&'a mut self) -> impl FnMut() + 'a {
        self.linked_items += 1;
        move || self.links_created += 1
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nitems: {}\nlinks: {}\n", self.linked_items, self.links_created)
    }
}
