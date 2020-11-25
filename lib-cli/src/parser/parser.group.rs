#[derive(Debug)]
pub struct Group {
    pub id: usize,
    pub parent: usize,
    pub name: String,
    pub structs: Vec<usize>,
    pub enums: Vec<usize>,
    pub groups: Vec<usize>,
}

impl Group {

    pub fn new(id: usize, parent: usize, name: String) -> Self {
        Group {
            id,
            parent,
            name,
            structs: vec![],
            enums: vec![],
            groups: vec![],
        }
    }

    pub fn bind_struct(&mut self, id: usize) {
        self.structs.push(id);
    }

    pub fn bind_enum(&mut self, id: usize) {
        self.enums.push(id);
    }

    pub fn bind_group(&mut self, id: usize) {
        self.groups.push(id);
    }

}
