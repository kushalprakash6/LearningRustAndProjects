#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(value: String) -> Self {
        Self {
            name: value,
            contents: Vec::new(),
        }
    }

    fn create_file(&mut self, name: String) {
        let f = File { name };
        self.contents.push(f);
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}


fn main() {
    let mut folder = Folder::new(String::from("my_folder"));

    folder.create_file(String::from("file1.txt"));
    folder.create_file(String::from("file2.txt"));

    println!("{:#?}", folder);

    let deleted = folder.delete_file(0);
    println!("{deleted:?}");

    println!("{:?}", folder);

    match folder.get_file(0) {
        Some(f) => println!("{:?}", f),
        None => println!("There was no file"),
    }
}
