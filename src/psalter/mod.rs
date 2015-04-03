extern crate toml;

use std::io::Read;
use std::fs::File;


#[derive(Debug)]
pub enum TextType{
    LXX,
    MT,
}

#[derive(Debug)]
pub struct Psalter{
    version: String,
    verse_delimiter: String,
    text_type: TextType,
}

#[derive(Debug)]
pub struct Psalm{
    number: usize,
    text: String,
}

//TODO: Make this not hard coded
static PSALTER_PATH: &'static str = "/home/aletheia/git/anthologion_rs/resources/example/bible/psalms.toml";

impl Psalter{
    pub fn new(version: String, verse_delimiter: String, text_type: TextType) -> Self{
        Psalter{ version: version, verse_delimiter: verse_delimiter, text_type: text_type }
    }

}

impl Psalm{
    pub fn new(number: usize, text: String) -> Self{
        Psalm{ number: number, text: text }
    }
}

impl ToString for Psalm{
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

pub fn get_psalm(target: usize) -> Psalm {
    let target = target - 1;
    let mut psalter_file = File::open(PSALTER_PATH).unwrap();
    let mut psalter_toml = String::new();
    psalter_file.read_to_string(&mut psalter_toml);
    let psalter: toml::Value = psalter_toml.parse().unwrap();
    //Get the psalm number
    let toml_path = "psalm.".to_string() + &target.to_string() +".number";
    let number = psalter.lookup(&toml_path).unwrap();
    let number = match *number {
        toml::Value::Integer(n) => n as usize,
        _ => 256 as usize,
    };
    //find the psalm text
    let toml_path = "psalm.".to_string() + &target.to_string() +".text";
    let text =  psalter.lookup(&toml_path).unwrap();
    let text = match *text{
        toml::Value::String(ref s) => s.clone(),
        _ => String::new(),
    };

//    let text = psalter.lookup("psalm.0.text").unwrap();
    //Psalm::new(number, text.to_string())
    Psalm::new(number, text)
}

#[test]
fn try_get_psalm(){
    let p = get_psalm(1);
    assert!(p.number == 1);
    assert!(p.text.contains("Blessed is the man"));
}
