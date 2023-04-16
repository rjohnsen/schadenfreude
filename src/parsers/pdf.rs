use regex::bytes::Regex;
use std::str;

#[derive(Debug, Default)]
pub struct PdfDocument {
    pub header: String,
    pub body: Vec<PdfObject>,
    pub xref: String,
    pub trailer: String,
}

#[derive(Debug, Default)]
pub struct PdfObject {
    id: u8,
    revision: u8,
    position: PdfObjectPosition
}

#[derive(Debug, Default)]
pub struct PdfObjectPosition {
    start: u8,
    end: u8
}

#[derive(PartialEq)]
enum PositionType {
    Start,
    End
}

impl PdfDocument {
    pub fn parse(&mut self, specimen: &crate::utils::specimen::Specimen) {
        println!("Parsing {}", specimen.name);
        self.header = self.parse_header(&specimen.content);
        self.parse_body(&specimen.content);

        println!("{:?}", self);
    }

    fn parse_header(&mut self, content: &Vec<u8>) -> String {
        let regx = Regex::new(r"^%(PDF.+)\n").unwrap(); 
        let caps = regx.captures(content).unwrap().get(1).unwrap().as_bytes();
        
        return str::from_utf8(caps).unwrap().to_string();
    }

    fn parse_body(&mut self, content: &Vec<u8>) {
        let start_pos: Vec<usize> = self.find_positions(r"\d+\s\d+\sobj", &content, PositionType::Start);
        let end_pos: Vec<usize> = self.find_positions(r"endobj", &content, PositionType::End);

        for index in 0..start_pos.len() {
            let mut data: Vec<u8> = (&content[start_pos[index]..end_pos[index]]).to_vec();
            
            let stream_start = self.find_positions(r"\nstream\n", &data, PositionType::End);
            let stream_end = self.find_positions(r"endstream", &data, PositionType::Start);
            
            println!("\n");

            if stream_start.len() == 0 && stream_end.len() == 0 {
                println!("{:?}", str::from_utf8(&data).unwrap());
        
            } else {
                // Note for further development
                // Data.Drain returns the deleted section. We can use that to further handle content
                data.drain(stream_start[0]..stream_end[0]);
                println!("{:?}", str::from_utf8(&data).unwrap());
            }
        }
    }

    fn find_positions(&mut self, regx_pattern: &str, content: &Vec<u8>, position_type: PositionType) -> Vec<usize> {
        let regx = Regex::new(regx_pattern).unwrap();
        let mut items: Vec<usize> = Vec::new();

        for caps in regx.captures_iter(content) {
            if position_type == PositionType::End {
                items.push(caps.get(0).unwrap().end());
            } else {
                items.push(caps.get(0).unwrap().start());
            }
        }

        return items;
    }
}