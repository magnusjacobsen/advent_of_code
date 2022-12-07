use std::io::BufRead;

/* struct File {
    size: u32,
    name: String,
}

struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>
}

impl File {
    fn new(name: String, size: u32) -> Self {
        Self {name, size}
    }
}

impl Folder {
    fn new(name: String) -> Self {
        Self {name, files: vec![], folders: vec![]}
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push(File::new(name, size));
    }

    fn add_folder(&mut self, name: String) {
        self.folders.push(Folder::new(name));
    }
} */

fn find(name: String, vec: &Vec<String>) -> i32 {
    for i in 0..vec.len() {
        if name == vec[i] {
            return i as i32;
        }
    }
    return -1;
}

fn main() {
    let lines = std::io::stdin().lock().lines().map(|line| line.unwrap().clone().split(" ").map(|x| x.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();
    
    let mut folder_names: Vec<String> = vec![];
    let mut contains: Vec<Vec<String>> = vec![];
    let mut file_sizes = vec![];
    let mut parents = vec![];
    let mut current_idx = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        match (line[0].as_str(), line[1].as_str()) {
            ("$", "cd") => {
                if line[2] == ".." {
                    current_idx = parents.pop().unwrap();
                } else {
                    let mut name_idx = find(line[2].clone(), &folder_names);
                    if name_idx == -1 {
                        name_idx = folder_names.len() as i32;
                        folder_names.push(line[2].clone());
                        file_sizes.push(0);
                        contains.push(vec![]);
                    } else {
                        //println!("we are here again!");
                    }
                    parents.push(current_idx);
                    current_idx = name_idx as usize;
                }
            },
            ("$", "ls") => {
                file_sizes[current_idx] = 0;
            },
            ("dir", dirname) => contains[current_idx].push(dirname.to_string()),
            (size, _filename) => {
                file_sizes[current_idx] += u64::from_str_radix(&size, 10).unwrap();
            },
        }
    }

    let mut acc = 0;
    for i in 0..file_sizes.len() {
        let size = rec_size(i, &folder_names, &contains, &file_sizes);
        if size <= 100000 {
            acc += size;
        }
    }
    println!("{}", acc);
}

// 5637318 too high
// 1037318 too low

fn rec_size(idx: usize, folder_names: &Vec<String>, contains: &Vec<Vec<String>>, file_sizes: &Vec<u64>) -> u64 {
    let mut acc = file_sizes[idx];
    for dirname in &contains[idx] {
        if acc > 100000 {
            return 100001;
        }
        let dir_idx = find(dirname.clone(), folder_names) as usize;
        acc += rec_size(dir_idx, folder_names, contains, file_sizes);
    }
    return acc;
}