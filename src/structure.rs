mod cmd_args;
mod read_xml;

fn main() {
    let file: String = cmd_args::exec();

    read_xml::exec(&file);
    println!("Hello, world!");
}
