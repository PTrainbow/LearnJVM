mod entry;

use std::path::PathBuf;
use crate::classpath::entry::{DirEntry, Entry, WildcardEntry};
use std::result::Result as StdResult;
use crate::error::Error;
use crate::error::Error::ClassNotFound;


pub type Result<T> = StdResult<T, Error>;

pub struct Classpath {
    boot_classpath: Box<dyn Entry>,
    ext_classpath: Box<dyn Entry>,
    user_classpath: Box<dyn Entry>,
}

impl Classpath {
    pub fn init_classpath(jre_classpath: PathBuf, user_classpath: PathBuf) -> Classpath {
        return Classpath {
            boot_classpath: Box::new(
                WildcardEntry::new(&jre_classpath.join("*"))
            ),
            ext_classpath: Box::new(
                WildcardEntry::new(&jre_classpath.join("ext").join("*"))
            ),
            user_classpath: Box::new(
                DirEntry {
                    path: user_classpath
                }
            ),
        };
    }

    pub(crate) fn load_class(&self, class_name: String) -> Result<Vec<u8>> {
        let real_name = class_name.replace(".", "/") + ".class";

        println!("try to find {} in boot classpath", class_name);
        let mut result = self.boot_classpath.read_class(&real_name);
        if result.is_ok() {
            println!("find {} in boot classpath, class content is:\n {:?}", class_name, result);
            // fs::write("test.class", boot_result);
            return result;
        }

        println!("try to find {} in ext classpath", class_name);
        result = self.ext_classpath.read_class(&real_name);
        if result.is_ok() {
            println!("find {} in ext classpath, class content is:\n {:?}", class_name, result);
            // fs::write("test.class", boot_result);
            return result;
        }

        println!("try to find {} in user classpath", class_name);
        result = self.user_classpath.read_class(&real_name);
        if result.is_ok() {
            println!("find {} in user classpath, class content is:\n {:?}", class_name, result);
            // fs::write("test.class", boot_result);
            return result;
        }
        println!(" class {} not found! ", class_name);
        return Err(ClassNotFound(class_name));
    }
}

