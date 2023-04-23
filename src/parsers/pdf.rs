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
    position: PdfObjectPosition,
    content: String
}

#[derive(Debug, Default)]
pub struct PdfObjectPosition {
    start: usize,
    end: usize
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
        self.body = self.parse_body(&specimen.content);

        self.display();
    }

    pub fn display(&mut self) {
        println!("PDF header: {}", self.header);

        for object in self.body.iter() {
            println!("\n-- Object {:?} -- Revision {:?} -- LOC: s:{:?} e:{:?} --------------------------\n", object.id, object.revision, object.position.start, object.position.end);
            println!("{:?}\n", object.content);
        }
    }

    fn parse_header(&mut self, content: &Vec<u8>) -> String {
        let regx = Regex::new(r"^%(PDF.+)(?:\r|\n)").unwrap();
        let caps = regx.captures(content).unwrap().get(1).unwrap().as_bytes();
        
        return str::from_utf8(caps).unwrap().to_string();
    }

    fn parse_body(&mut self, content: &Vec<u8>) -> Vec<PdfObject> {
        let mut object_list: Vec<PdfObject> = Vec::new();
        let start_pos: Vec<usize> = self.find_positions(r"\d+\s\d+\sobj", &content, PositionType::Start);
        let end_pos: Vec<usize> = self.find_positions(r"endobj", &content, PositionType::End);

        for index in 0..start_pos.len() {
            let mut pdf_object = PdfObject::default();

            // Get object as bytes
            let raw_object: Vec<u8> = (&content[start_pos[index]..end_pos[index]]).to_vec();

            // Get Object ID and Revision
            let obj_regx = Regex::new(r"^(?P<oid>\d+)\s(?P<rev>\d+)\sobj").unwrap();
            let obj_captures = obj_regx.captures(&raw_object).unwrap();
            
            // Prepare PDF object object (haha, ok ok)
            pdf_object.id =  std::str::from_utf8(&obj_captures.name("oid").unwrap().as_bytes()).unwrap().parse().unwrap();
            pdf_object.revision = std::str::from_utf8(&obj_captures.name("rev").unwrap().as_bytes()).unwrap().parse().unwrap();
            pdf_object.position = PdfObjectPosition{ start: start_pos[index], end: end_pos[index]};

            // Determine if object contain stream
            let streamstart_pos: Vec<usize> = self.find_positions(r">>stream", &raw_object, PositionType::End);
            let streamend_pos: Vec<usize> = self.find_positions(r"endstream", &raw_object, PositionType::Start);

            if streamstart_pos.len() > 0 && streamend_pos.len() > 0 {
                // println!("Got stream");
            } else {
                if raw_object.is_ascii() == true {
                    pdf_object.content = String::from_utf8(raw_object).unwrap();
                } else {
                    // Probably decode this thingy
                    pdf_object.content = String::from_utf8_lossy(&raw_object).to_string();
                }
            }
            
            object_list.push(pdf_object);
            
            /*
            println!("---------------------------------------------------------------");
            println!("\nObject #{:?}r{:?} @{:?}:{:?}", pdf_object.id,pdf_object.revision, pdf_object.position.start, pdf_object.position.end);
            println!("{}\n", pdf_object.content);
            */ 

            /*
            // Determine if ASCII
            if raw_object.is_ascii() == true {
                println!("obj {:?}", oid);
                println!("\t{:?}", std::str::from_utf8(&raw_object));
            } else {
                println!("obj {} => !ASCII", oid);
                println!("\t{:?}", String::from_utf8_lossy(&raw_object));
            }
            */ 


            /*
            let mut data: Vec<u8> = (&content[start_pos[index]..end_pos[index]]).to_vec();
            
            let stream_start = self.find_positions(r">>stream", &data, PositionType::End);
            let stream_end = self.find_positions(r"endstream", &data, PositionType::Start);
            
            // Extract object metadata
            let obj_regx = Regex::new(r"^(?P<oid>\d+)\s(?P<rev>\d+)\sobj").unwrap();
            let obj_captures = obj_regx.captures(&data).unwrap();

            // println!("{:?} => {:?}", &obj_captures["oid"], &obj_captures["rev"]);
            let oid: u8 = std::str::from_utf8(&obj_captures.name("oid").unwrap().as_bytes()).unwrap().parse().unwrap();
            let rev: u8 = std::str::from_utf8(&obj_captures.name("rev").unwrap().as_bytes()).unwrap().parse().unwrap();

            println!("OID: {:?}", oid);

            if stream_start.len() == 0 && stream_end.len() == 0 {
                if (data.is_ascii() == true) {
                    let result = String::from_utf8(data);
                    println!("{:?}", true);
                } else {
                    println!("{:?}", &data);
                }
                
                // println!("{:?}", str::from_utf8(&data));
            } else {
                // Note for further development
                // Data.Drain returns the deleted section. We can use that to further handle content
                // data.drain(stream_start[0]..stream_end[0]);
                // println!("{:?}", str::from_utf8(&data).unwrap());
            }
            */
        }

        return object_list;
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