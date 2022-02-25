extern crate xml;


use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};


mod vec_id;


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
                if name.local_name == "section"
                {
                    println!("{:?}", vec_id::exec(&attributes))
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
