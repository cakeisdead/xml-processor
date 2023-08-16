use std::{io::{self, Read}, fs::{File, self}};
use std::path::{PathBuf, Path};
use time::{OffsetDateTime, format_description};
use std::time::SystemTime;
use xmltree::Element;
const DATE_FORMAT_STR: &'static str = "EXEC_[day]_[month]_[year]-[hour]_[minute]";
fn main() {
    
    // Generate execution Id
    let dt_fmt = format_description::parse(DATE_FORMAT_STR).unwrap();
    let now: OffsetDateTime = SystemTime::now().into();
    let execution_id = now.format(&dt_fmt).unwrap();
    println!("Execution Id: {:?}", &execution_id);

    let food_search_list = vec!["Colombian Waffles", "Fire Toast"];
    let mut data: Vec<String> = vec![];

    println!("## Searched Items:");
    for item in &food_search_list {
        println!("{}", item);
    }

    println!("\n## Files:");
    let files = fs::read_dir("./files").unwrap();
    let out_path = Path::new("./files").join(&execution_id);
    fs::create_dir(out_path).unwrap();

    for file in files {
        let f = file.as_ref();
        let file_name = f.unwrap().file_name();
        let xml_path = f.unwrap().path();
        
        // skip if not an xml file
        if !file_name.to_str().unwrap().ends_with(".xml") 
        {
            continue;
        }

        println!("File: {:?}", file_name);
        read_xml(&xml_path, &mut data).unwrap();
    }

    println!("\n## Search results:");
    for item in &food_search_list {
        let qty = data.iter().filter(|&n| *n == item.to_string()).count();
        println!("{0} x{1}", item, qty);
    }
}

fn read_xml(xml_path: &PathBuf, data: &mut Vec<String>) -> io::Result<()> {
    let mut f = File::open(xml_path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    
    let root_node = Element::parse(buffer.as_bytes()).unwrap();
    
    for child in root_node.children {
        let sub_child = child.as_element().unwrap().get_child("name");
        for sub_sub in &sub_child.unwrap().children 
        {
            let food = sub_sub.as_text().unwrap();
            data.push(food.to_string());
            println!("\t Added {0} into summary vector.", food);
        }
    }
    Ok(())
}
