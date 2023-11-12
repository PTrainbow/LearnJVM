mod classpath;
mod error;
mod classfile;
mod runtime;

use std::borrow::Borrow;
use std::cell::Cell;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use classpath::Classpath;
use structopt::StructOpt;
use crate::classfile::class_reader::{get_class_name, Reader};
use crate::classfile::ClassFile;
use crate::error::Error;
use crate::runtime::{Frame, LocalVars, Object, OperandStack, Slot, Thread};

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
            let this_class = get_class_name(&classfile.constant_pool, &classfile.this_class);
            let super_class = get_class_name(&classfile.constant_pool, &classfile.super_class);
            println!("parse classfile successful");
            println!("version: {}", classfile.major_version);
            println!("constants count: {}", classfile.constant_pool_count);
            println!("access_flags: {:#x}", classfile.access_flags);
            println!("this class: {:?}", this_class);
            println!("super class: {:?}", super_class);
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
    test_ch_04();
}

fn test_ch_04() {
    let frame = Frame::new_frame(1024, 1024);
    test_local_vars(frame.local_vars);
    test_operand_stack(frame.operand_stack);
}

fn test_operand_stack(mut ops: Box<OperandStack>) {
    ops.deref_mut().push_int(100);
    ops.deref_mut().push_int(-100);
    ops.deref_mut().push_long(2997924580);
    ops.deref_mut().push_long(-2997924580);
    ops.deref_mut().push_float(3.1415926);
    ops.deref_mut().push_double(2.71828182845);
    ops.deref_mut().push_ref(None);
    println!("{:?}", ops.deref_mut().pop_ref());
    println!("{}", ops.deref_mut().pop_double());
    println!("{}", ops.deref_mut().pop_float());
    println!("{}", ops.deref_mut().pop_long());
    println!("{}", ops.deref_mut().pop_long());
    println!("{}", ops.deref_mut().pop_int());
    println!("{}", ops.deref_mut().pop_int());
}

fn test_local_vars(mut vars: LocalVars) {
    vars.set_int(0, 100);
    vars.set_int(1, -100);
    vars.set_long(2, 2997924580);
    vars.set_long(4, -2997924580);
    vars.set_float(6, 3.1415926);
    vars.set_double(7, 2.71828182845);
    vars.set_ref(9, None);
    println!("{}", vars.get_int(0));
    println!("{}", vars.get_int(1));
    println!("{}", vars.get_long(2));
    println!("{}", vars.get_long(4));
    println!("{}", vars.get_float(6));
    println!("{}", vars.get_double(7));
    println!("{:?}", vars.get_ref(9));

    let test = -10;
    println!("signed-extend = {:016x}, zero extend = {:016x}", (test as i32) as i64, (test as u32) as i64);

}
