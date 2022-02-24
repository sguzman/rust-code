extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String
{
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(
            String::with_capacity(
                size*INDENT.len()),
                |r, s| r + s)
}

pub fn exec(path: &String)
{
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser
    {
        match e
        {
            Ok(XmlEvent::StartElement { name, attributes, .. }) =>
            {
                if name.local_name == "section"
                {
                    for attr in 0..(attributes.len())
                    {
                        if attributes[attr].name.local_name == "identifier"
                        {
                            println!("{:?}", attributes[attr].value);
                            break;
                        }

                    }
                }
            }
            Err(e) =>
            {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
