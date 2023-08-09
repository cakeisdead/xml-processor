use std::{io::{self, Read}, fs::{File, self}};
use std::path::PathBuf;
use xmltree::Element;

fn main() {
    let food_search_list = vec!["Colombian Waffles", "Fire Toast"];
    println!("## Searched Items:");
    for item in food_search_list {
        println!("{}", item);
    }

    println!("## Files:");
    let files = fs::read_dir("./files").unwrap();
    
    for file in files {
        let f = file.as_ref();
        let file_name = f.unwrap().file_name();
        let xml_path = f.unwrap().path();
        println!("File: {:?}", file_name);
        read_xml(&xml_path).unwrap();
    }
}

fn read_xml(xml_path: &PathBuf) -> io::Result<()> {
    let mut f = File::open(xml_path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    
    let root_node = Element::parse(buffer.as_bytes()).unwrap();
    
    for child in root_node.children {
        let sub_child = child.as_element().unwrap().get_child("name");
        for sub_sub in &sub_child.unwrap().children 
        {
            println!("\t{}", sub_sub.as_text().unwrap());
        }
    }
    Ok(())
}
