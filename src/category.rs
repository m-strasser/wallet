use std::fmt;

static mut curID: i64 = 0;

#[derive(Debug)]
pub struct Category {
    pub ID: i64,
    pub name: String,
    pub description: String
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

impl Category {
    pub fn new(name: String, description: String) -> Category {
        unsafe {
            curID+=1;

            Category {
                ID: curID,
                name: name,
                description: description
            }
        }
    }
}
