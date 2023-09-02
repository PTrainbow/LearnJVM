use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="LearnJVM", usage="Usage: LearnJVM [-options] class [args...]")]
pub struct Options {
    #[structopt(long="version", help="print version message")]
    version_flag: bool,
    #[structopt( long="classpath", help= "classpath",  takes_value=true)]
    classpath: Option<String>,
    #[structopt( long="cp", help= "classpath",  takes_value=true)]
    cp: Option<String>,
    #[structopt(takes_value=true)]
    class: Option<String>,
    #[structopt(takes_value=true, multiple=true)]
    args: Vec<String>
}

fn main() {
    let options = Options::from_args();
    if options.version_flag {
        print!("version: 0.0.1\n");
        return;
    } else if options.classpath.is_some() || options.cp.is_some() {
        println!("parse input {:?}\n", options);
    } else {
        print!("Usage: LearnJVM [-options] class [args...]\n")
    }
}
