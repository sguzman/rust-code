extern crate xml;


use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

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
                match name.local_name.as_str()
                {
                    "section" =>
                    {
                        for attr in attributes
                        {
                            match attr.name.local_name.as_str()
                            {
                                "identifier"=>
                                {
                                    println!("{}", attr.value);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
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
