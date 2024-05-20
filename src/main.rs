mod classpath;
mod error;
mod classfile;
mod runtime;
mod instructions;


use std::cell::Cell;
use std::path::PathBuf;
use classfile::ClassFile;
use classpath::Classpath;
use structopt::StructOpt;
use crate::classfile::class_reader::{get_class_name, get_utf8,Reader};
use crate::runtime::{interpret, Frame, LocalVars, OperandStack, Thread};

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
        println!("version: 0.0.1");
    } else if options.jre.is_some() && options.class.is_some() {
        println!("parse input:\n {:?}", options);
        parse_internal(options);
    } else {
        println!("Usage: LearnJVM [-options] class [args...]")
    }
}

fn parse_internal(options: Options) {
    let jre_lib_dir = PathBuf::from(options.jre.unwrap()).join("lib");
    if options.cp.is_none() && options.classpath.is_none() {
        println!("{}", error::Error::ClasspathNotSet());
        return;
    }
    let user_classpath =  if options.cp.is_none() {
        options.classpath.unwrap()
    } else {
        options.cp.unwrap()
    };
    let classpath = Classpath::init_classpath(jre_lib_dir, PathBuf::from(user_classpath));
    let class = classpath.load_class(options.class.unwrap());
    let reader = Reader {
        content: class.unwrap(),
        cursor: Cell::new(0),
    };
    test_ch_04();
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
            for field_info in &classfile.fields_info {
                println!("  {}", field_info.name);
            }
            println!("methods count: {}", classfile.methods_count);
            for method_info in &classfile.methods_info {
                println!("  {}", method_info.name);
            }
            println!("attributes count: {}", classfile.attributes_count);
            test_ch_05(&classfile);
        }
    }
}

fn test_ch_05(classfile:&ClassFile) {
   for method in &classfile.methods_info {
       if method.name == "main" {
           let descriptor_name = get_utf8(&classfile.constant_pool, &method.descriptor_index).unwrap();
           if descriptor_name == "([Ljava/lang/String;)V" {
               interpret(method);
               return;
           }
       }
   }
   println!("not found main function!!");
}

fn test_ch_04() {
    let mut thread = Thread::new_thread();
    let frame = Frame::new_frame(std::ptr::addr_of_mut!(thread), 1024, 1024);
    test_local_vars(frame.local_vars);
    test_operand_stack(frame.operand_stack);
}

fn test_operand_stack(mut ops: OperandStack) {
    ops.push_int(100);
    ops.push_int(-100);
    ops.push_long(2997924580);
    ops.push_long(-2997924580);
    ops.push_float(3.1415926);
    ops.push_double(2.71828182845);
    ops.push_ref(None);
    println!("{:?}", ops.pop_ref());
    println!("{}", ops.pop_double());
    println!("{}", ops.pop_float());
    println!("{}", ops.pop_long());
    println!("{}", ops.pop_long());
    println!("{}", ops.pop_int());
    println!("{}", ops.pop_int());
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
    println!("{:?}", vars.get_ref(9));
    println!("{}", vars.get_double(7));

    let test = -10;
    println!("signed-extend = {:016x}, zero extend = {:016x}", (test as i32) as i64, (test as u32) as i64);

}
