mod classpath;
mod error;
mod classfile;

use std::cell::Cell;
use std::path::PathBuf;
use classpath::Classpath;
use structopt::StructOpt;
use crate::classfile::class_reader::{get_class_name, Reader};
use crate::classfile::ClassFile;
use crate::error::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "LearnJVM", usage = "Usage: LearnJVM [-options] class [args...]")]
pub struct Options {
    #[structopt(long = "version", help = "print version message")]
    version_flag: bool,
    #[structopt(long = "classpath", help = "classpath", takes_value = true)]
    classpath: Option<String>,
    #[structopt(long = "cp", help = "classpath", takes_value = true)]
    cp: Option<String>,
    #[structopt(long = "jre", help = "jre path", takes_value = true)]
    jre: Option<String>,
    #[structopt(takes_value = true)]
    class: Option<String>,
    #[structopt(takes_value = true, multiple = true)]
    _args: Vec<String>,
}

fn main() {
    let options = Options::from_args();
    if options.version_flag {
        print!("version: 0.0.1\n");
    } else if options.jre.is_some() && options.class.is_some() {
        println!("parse input:\n {:?}", options);
        parse_internal(options);
    } else {
        print!("Usage: LearnJVM [-options] class [args...]\n")
    }
}

fn parse_internal(options: Options) {
    let jre_lib_dir = PathBuf::from(options.jre.unwrap()).join("lib");
    let user_classpath = if options.classpath.is_some() {
        Some(PathBuf::from(options.classpath.unwrap()))
    } else if options.cp.is_some() {
        Some(PathBuf::from(options.cp.unwrap()))
    } else {
        None
    };
    let classpath = Classpath::init_classpath(jre_lib_dir, user_classpath);
    let class = classpath.load_class(options.class.unwrap());
    let reader = Reader {
        content: class.unwrap(),
        cursor: Cell::new(0),
    };
    let result = reader.parse_classfile();
    match result {
        Err(error) => {
            println!("failed {}", error)
        }

        Ok(classfile) => {
            println!("parse classfile successful");
            println!("version: {}", classfile.major_version);
            println!("constants count: {}", classfile.constant_pool_count);
            println!("access_flags: {:#x}", classfile.access_flags);
            println!("this class: {}", get_class_name(&classfile.constant_pool, &classfile.this_class).unwrap());
            println!("super class: {}", get_class_name(&classfile.constant_pool, &classfile.super_class).unwrap());
            println!("interfaces: {:?}", classfile.interfaces);
            println!("fields count: {}", classfile.fields_count);
            for field_info in classfile.fields_info {
                println!("  {}", field_info.name);
            }
            println!("methods count: {}", classfile.methods_count);
            for method_info in classfile.methods_info {
                println!("  {}", method_info.name);
            }
        }
    }
}
