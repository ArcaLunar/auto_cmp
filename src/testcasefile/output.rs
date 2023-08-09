use crate::testcase_result::TestCaseResult;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

pub struct OutputFile {
    testcase_id: usize,
    dir_path: String,
    file_name: String,
    file_extension: String,
    full_path: String,
}

impl OutputFile {
    /// Initialize a new `OutputFile` object, which points to\
    /// standard output file.
    pub fn new(work_dir: &String) -> Self {
        Self {
            testcase_id: 0,
            dir_path: (*work_dir).clone(),
            file_name: String::from("output"),
            file_extension: String::from(".out"),
            full_path: String::new(),
        }
    }

    pub fn get_full_path(&self) -> &String {
        &self.full_path
    }

    /// Accepts two `OutputFile` objects as input, and reads the contents\
    /// judges whether they're same(ignoring `'\r'` and `'\n'`)\
    /// Returns `Result<TestCaseResult>`\
    /// Note that for the time being, this function may panic when files can't\
    /// open properly. ***Further error handling will be added.***
    pub fn compare(o1: &OutputFile, o2: &OutputFile) -> std::io::Result<TestCaseResult> {
        // Opens the files, panickes if error opening
        let mut f1 = File::open(o1.get_full_path())?;
        let mut f2 = File::open(o2.get_full_path())?;

        // Reads contents to vector `Vec<u8>`
        // Panickes if error reading the file
        let mut v1 = String::new();
        let mut v2 = String::new();
        f1.read_to_string(&mut v1)?;
        f2.read_to_string(&mut v2)?;

        // Eliminating '\r', '\n' from strings and compares whether same
        v1 = v1.replace("\r", "").replace("\n", "");
        v2 = v2.replace("\r", "").replace("\n", "");

        // Judges whether the two files are the same
        match v1.partial_cmp(&v2).unwrap() {
            Ordering::Equal => Ok(TestCaseResult::Accepted),
            _ => Ok(TestCaseResult::WrongAnswer ("WA".to_string())),
        }
    }

    /// Updates the full path of the file with given information
    pub fn update(&mut self) {
        self.full_path.push_str(&self.dir_path[..]);
        self.full_path.push_str("/");
        self.full_path.push_str(&self.file_name[..]);
        self.full_path.push_str(&self.testcase_id.to_string()[..]);
        self.full_path.push_str(".");
        self.full_path.push_str(&self.file_extension[..]);
    }
}
