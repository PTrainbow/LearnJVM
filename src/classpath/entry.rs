use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;
use zip::ZipArchive;
use std::result::Result as StdResult;

pub use crate::error::Error;

pub type Result<T> = StdResult<T, Error>;

pub trait Entry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>>;
    fn print_path(&self);
}

pub struct DirEntry {
    pub(crate) path: PathBuf,
}

pub struct ZipEntry {
    path: PathBuf,
}

pub struct WildcardEntry {
    entry_list: Vec<Box<dyn Entry>>,
}

impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>> {
        for entry in WalkDir::new(&self.path)
            .into_iter()
            .filter_map(StdResult::ok)
            .filter(|e| !e.file_type().is_dir()) {
            // println!("name is {}", entry.path().display());
            if let Some(file_name) = entry.path().to_str() {
                if file_name.contains(class_name) {
                    let mut data = Vec::new();
                    File::open(file_name)?.read_to_end(&mut data)?;
                    return Ok(data);
                }
            }
        }
        Err(Error::ClassNotFound(String::from(class_name)))
    }

    fn print_path(&self) {
        println!("path is {}", self.path.display())
    }
}

impl Entry for ZipEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>> {
        let archive1 = File::open(&self.path)?;
        let mut archive = ZipArchive::new(archive1)?;
        for idx in 0..archive.len() {
            let mut entry = archive.by_index(idx)?;
            if class_name == entry.name() {
                println!("find id is {}", entry.name());
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;
                return Ok(data);
            }
            // println!("name is {}", entry.name())
        }
        Err(Error::ClassNotFound(String::from(class_name)))
    }

    fn print_path(&self) {
        println!("path is {}", self.path.display())
    }
}

impl Entry for WildcardEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>> {
        for entry in &self.entry_list {
            let class_content = entry.read_class(class_name);
            if class_content.is_ok() {
                return class_content;
            }
        }
        Err(Error::ClassNotFound(String::from(class_name)))
    }

    fn print_path(&self) {
        for entry in &self.entry_list {
            entry.print_path()
        }
    }
}

impl WildcardEntry {
    pub fn new(path: &PathBuf) -> WildcardEntry {
        let mut result = WildcardEntry {
            entry_list: Vec::new(),
        };
        let path_slice = path.to_str().unwrap();
        let base_dir = &path_slice[0..path_slice.len() - 1];
        for entry in WalkDir::new(base_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(StdResult::ok)
            .filter(|e| !e.file_type().is_dir()) {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".jar") || file_name.ends_with(".JAR") {
                    result.entry_list.push(Box::new(ZipEntry {
                        path: PathBuf::from(entry.path())
                    }))
                }
            }
        }
        return result;
    }
}