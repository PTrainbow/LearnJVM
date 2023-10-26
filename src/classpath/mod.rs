mod errors;
mod entry;

use std::path::{PathBuf};
use crate::classpath::entry::{DirEntry, Entry, WildcardEntry};


pub struct Classpath {
    boot_classpath: Box<dyn Entry>,
    ext_classpath: Box<dyn Entry>,
    user_classpath: Box<dyn Entry>,
}

impl Classpath {
    pub fn init_classpath(jre_classpath: PathBuf, user_classpath: Option<PathBuf>) -> Classpath {
        return Classpath {
            boot_classpath: Box::new(
                WildcardEntry::new(&jre_classpath.join("*"))
            ),
            ext_classpath: Box::new(
                WildcardEntry::new(&jre_classpath.join("ext").join("*"))
            ),
            user_classpath: Box::new(
                DirEntry {
                    path: user_classpath.unwrap_or(PathBuf::new())
                }
            ),
        };
    }

    pub(crate) fn load_class(&self, class_name: String) {
        let real_name = class_name.replace(".", "/") + ".class";

        println!("try to find {} in boot classpath", class_name);
        let boot_result = self.boot_classpath.read_class(&real_name);
        if boot_result.is_ok() {
            println!("find {} in boot classpath, class content is:\n {:?}", class_name, boot_result);
            // fs::write("test.class", boot_result);
            return;
        }

        println!("try to find {} in ext classpath", class_name);
        let ext_result = self.ext_classpath.read_class(&real_name);
        if ext_result.is_ok() {
            println!("find {} in ext classpath, class content is:\n {:?}", class_name, ext_result);
            // fs::write("test.class", boot_result);
            return;
        }

        println!("try to find {} in user classpath", class_name);
        let user_result = self.user_classpath.read_class(&real_name);
        if user_result.is_ok() {
            println!("find {} in user classpath, class content is:\n {:?}", class_name, user_result);
            // fs::write("test.class", boot_result);
            return;
        }
        println!(" class {} not found! ", class_name);
    }
}

