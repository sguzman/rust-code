extern crate xml;


use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};


mod id;
mod section;

pub fn exec(path: &String)
{
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser
    {
        match e
        {
            Ok(XmlEvent::StartElement { name, attributes, .. }) =>
            {
                if section::exec(&name)
                {
                    match id::exec(&attributes) {
                        Some(string) =>
                        {
                            println!("{}", string);
                        }
                        None => {}
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
