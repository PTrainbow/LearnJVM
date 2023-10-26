mod classpath;

use std::path::PathBuf;
use classpath::Classpath;
use structopt::StructOpt;

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
    } else if options.cp.is_some()  {
        Some(PathBuf::from(options.cp.unwrap()))
    } else {
        None
    };
    let classpath = Classpath::init_classpath(jre_lib_dir, user_classpath);
    classpath.load_class(options.class.unwrap())
}
