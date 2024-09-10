use std::{collections::HashMap, fs};

type Vec2 = (isize, isize);
type Vec3 = (isize, isize, isize);
type Vec4 = (isize, isize, isize, isize);

pub fn unicode_to_char(unicode: &str) -> char {
    let nums: String = unicode.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    u8::from_str_radix(&nums, 16).unwrap_or(0) as char
}

#[derive(Clone)]
pub struct Character {
    name: char,
    encoding: isize,
    s_width: Vec2,
    d_width: Vec2,
    bbx: Vec4,
    bytes: Vec<u8>,
}

impl Character {
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn d_width(&self) -> Vec2 {
        self.d_width
    }
}

#[derive(Clone)]
enum FontProperties {
    FontAscent(usize),
    FontDescent(usize),
}

#[derive(Clone)]
pub struct Font {
    version: f32,
    description: String,
    size: Vec3,
    bounding_box: Vec4,
    properties: Vec<FontProperties>,
    char_count: usize,
    characters: HashMap<char, Character>,
}

impl Font {
    pub fn new(path: &str) -> Result<Self, &'static str> {
        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(_) => return Err("unable to open font at provided path"),
        };

        let mut reading_bitmap: bool = false;

        let mut version: Option<f32> = None;
        let mut description: Option<String> = None;
        let mut size: Option<Vec3> = None;
        let mut bounding_box: Option<Vec4> = None;
        let mut properties: Vec<FontProperties> = Vec::new();
        let mut char_count: Option<usize> = None;
        let mut characters: HashMap<char, Character> = HashMap::new();

        let mut char_name: Option<char> = None;
        let mut char_encoding: Option<isize> = None;
        let mut char_swidth: Option<Vec2> = None;
        let mut char_dwidth: Option<Vec2> = None;
        let mut char_bbx: Option<Vec4> = None;
        let mut char_bytes: Vec<u8> = Vec::new();

        for line in file.lines() {
            let contents: Vec<&str> = line.split(' ').collect();
            match contents[0] {
                "STARTFONT" => version = Some(contents[1].parse().unwrap()),
                "FONT" => description = Some(contents[1].to_string()),
                "SIZE" => {
                    size = Some((
                        contents[1].parse().unwrap(),
                        contents[2].parse().unwrap(),
                        contents[3].parse().unwrap(),
                    ))
                }
                "FONTBOUNDINGBOX" => {
                    bounding_box = Some((
                        contents[1].parse().unwrap(),
                        contents[2].parse().unwrap(),
                        contents[3].parse().unwrap(),
                        contents[4].parse().unwrap(),
                    ))
                }
                "FONT_ASCENT" => {
                    properties.push(FontProperties::FontAscent(contents[1].parse().unwrap()))
                }
                "FONT_DESCENT" => {
                    properties.push(FontProperties::FontDescent(contents[1].parse().unwrap()))
                }
                "CHARS" => char_count = Some(contents[1].parse().unwrap()),
                "STARTCHAR" => char_name = Some(unicode_to_char(contents[1])),
                "ENCODING" => char_encoding = Some(contents[1].parse().unwrap()),
                "SWIDTH" => {
                    char_swidth = Some((contents[1].parse().unwrap(), contents[2].parse().unwrap()))
                }
                "DWIDTH" => {
                    char_dwidth = Some((contents[1].parse().unwrap(), contents[2].parse().unwrap()))
                }
                "BBX" => {
                    char_bbx = Some((
                        contents[1].parse().unwrap(),
                        contents[2].parse().unwrap(),
                        contents[3].parse().unwrap(),
                        contents[4].parse().unwrap(),
                    ))
                }
                "BITMAP" => reading_bitmap = true,
                "ENDCHAR" => {
                    reading_bitmap = false;
                    let c = Character {
                        name: char_name.unwrap(),
                        encoding: char_encoding.unwrap(),
                        s_width: char_swidth.unwrap(),
                        d_width: char_dwidth.unwrap(),
                        bbx: char_bbx.unwrap(),
                        bytes: char_bytes,
                    };
                    characters.insert(char_name.unwrap(), c);

                    char_name = None;
                    char_encoding = None;
                    char_swidth = None;
                    char_dwidth = None;
                    char_bbx = None;
                    char_bytes = Vec::new();
                }
                _ => {
                    if reading_bitmap {
                        char_bytes.push(u8::from_str_radix(line, 16).unwrap());
                    }
                }
            }
        }

        Ok(Self {
            version: version.unwrap(),
            description: description.unwrap(),
            size: size.unwrap(),
            bounding_box: bounding_box.unwrap(),
            properties,
            char_count: char_count.unwrap(),
            characters,
        })
    }
    pub fn bounding_box(&self) -> Vec4 {
        self.bounding_box
    }
    pub fn get_character(&self, c: char) -> &Character {
        // TODO: Return a default character if one not found
        self.characters.get(&c).unwrap()
    }
}
