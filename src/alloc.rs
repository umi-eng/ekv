use crate::config::*;

/// Page allocator
pub struct Allocator {
    used_pages: [bool; PAGE_COUNT], // TODO use a bitfield
    next_page_id: PageID,
}

impl Allocator {
    pub fn new() -> Self {
        Self {
            used_pages: [false; PAGE_COUNT],
            next_page_id: 0, // TODO make random to spread out wear
        }
    }

    pub fn allocate(&mut self) -> PageID {
        let start = self.next_page_id;
        loop {
            let p = self.next_page_id;
            self.next_page_id += 1;
            if self.next_page_id == PAGE_COUNT as _ {
                self.next_page_id = 0;
            }

            if !self.used_pages[p as usize] {
                self.used_pages[p as usize] = true;
                return p;
            }

            if self.next_page_id == start {
                panic!("No free pages"); // TODO
            }
        }
    }

    pub fn mark_allocated(&mut self, page_id: PageID) {
        assert!(!self.used_pages[page_id as usize]);
        self.used_pages[page_id as usize] = true;
    }

    pub fn free(&mut self, page_id: PageID) {
        assert!(self.used_pages[page_id as usize]);
        self.used_pages[page_id as usize] = false;
    }

    #[cfg(test)]
    pub fn is_allocated(&self, page_id: PageID) -> bool {
        self.used_pages[page_id as usize]
    }
}
