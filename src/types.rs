#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl From<Message> for Vec<u8> {
    fn from(val: Message) -> Self {
        let mut res = Vec::new();

        let header: [u8; 12] = val.header.into();

        res.extend_from_slice(&header);

        for question in val.questions {
            let question: Vec<u8> = question.into();
            res.extend_from_slice(&question);
        }

        for answer in val.answers {
            let answer: Vec<u8> = answer.into();
            res.extend_from_slice(&answer);
        }

        res
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            panic!("Unable to parse message as it is too short")
        };

        let mut pointer = 0;

        let header = &value[pointer..12];
        let mut header_slice: [u8; 12] = [0; 12];
        header_slice.clone_from_slice(header);
        let header = Header::try_from(header_slice)?;

        pointer += 12;

        println!("Parsed header: {:?}", header);

        let mut questions = Vec::new();

        if header.question_count == 1 {
            let remaining_buf = &value[pointer..];
            let question = Question::try_from(remaining_buf.to_vec())?;

            // null terminating character for the string, 2 bytes for the type and 2 bytes for the class
            pointer += question.name.len() + 6;

            println!("Parsed Question: {:?}", question);

            questions.push(question);
        }

        let mut answers = Vec::new();

        if header.answer_count == 1 {
            let remaining_buf = &value[pointer..];
            let answer: Answer = Answer::try_from(remaining_buf.to_vec())?;

            answers.push(answer);
        }

        Ok(Message {
            header,
            questions,
            answers,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperationCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
}

impl TryFrom<u8> for OperationCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperationCode::Query),
            1 => Ok(OperationCode::IQuery),
            2 => Ok(OperationCode::Status),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl TryFrom<u8> for ResponseCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResponseCode::NoError),
            1 => Ok(ResponseCode::FormatError),
            2 => Ok(ResponseCode::ServerFailure),
            3 => Ok(ResponseCode::NameError),
            4 => Ok(ResponseCode::NotImplemented),
            5 => Ok(ResponseCode::Refused),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub qr_indicator: bool,
    pub op_code: OperationCode,
    pub authoritative_answer: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub response_code: ResponseCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16,
}

impl From<Header> for [u8; 12] {
    fn from(val: Header) -> Self {
        let mut res = [0; 12];

        res[0] = val.id.to_be_bytes()[0];
        res[1] = val.id.to_be_bytes()[1];
        res[2] = (val.qr_indicator as u8) << 7
            | (val.op_code as u8) << 3
            | (val.authoritative_answer as u8) << 2
            | (val.truncation as u8) << 1
            | (val.recursion_desired as u8);
        res[3] = (val.recursion_available as u8) << 7 | (val.response_code as u8);
        res[4] = val.question_count.to_be_bytes()[0];
        res[5] = val.question_count.to_be_bytes()[1];
        res[6] = val.answer_count.to_be_bytes()[0];
        res[7] = val.answer_count.to_be_bytes()[1];
        res[8] = val.authority_count.to_be_bytes()[0];
        res[9] = val.authority_count.to_be_bytes()[1];
        res[10] = val.additional_count.to_be_bytes()[0];
        res[11] = val.additional_count.to_be_bytes()[1];

        res
    }
}

impl TryFrom<[u8; 12]> for Header {
    type Error = ();

    fn try_from(value: [u8; 12]) -> Result<Self, Self::Error> {
        let id: u16 = (value[0] as u16) << 8 | (value[1] as u16);
        let qr_indicator = (value[2] >> 7 & 1) == 1;

        let op_code = OperationCode::try_from(value[2] >> 3 & 0b00001111)?;
        let authoritative_answer = (value[2] >> 2 & 1) == 1;
        let truncation = (value[2] >> 1 & 1) == 1;
        let recursion_desired = (value[2] & 1) == 1;
        let recursion_available = (value[3] >> 7 & 1) == 1;
        let response_code = ResponseCode::try_from(value[3] & 0b00001111)?;
        let question_count = (value[4] as u16) << 8 | (value[5] as u16);
        let answer_count = (value[6] as u16) << 8 | (value[7] as u16);
        let authority_count = (value[8] as u16) << 8 | (value[9] as u16);
        let additional_count = (value[10] as u16) << 8 | (value[11] as u16);

        Ok(Header {
            id,
            qr_indicator,
            op_code,
            authoritative_answer,
            truncation,
            recursion_desired,
            recursion_available,
            response_code,
            question_count,
            answer_count,
            authority_count,
            additional_count,
        })
    }
}

#[derive(Debug)]
pub enum QuestionType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ALL = 255,
}

impl TryFrom<u16> for QuestionType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QuestionType::A),
            2 => Ok(QuestionType::NS),
            3 => Ok(QuestionType::MD),
            4 => Ok(QuestionType::MF),
            5 => Ok(QuestionType::CNAME),
            6 => Ok(QuestionType::SOA),
            7 => Ok(QuestionType::MB),
            8 => Ok(QuestionType::MG),
            9 => Ok(QuestionType::MR),
            10 => Ok(QuestionType::NULL),
            11 => Ok(QuestionType::WKS),
            12 => Ok(QuestionType::PTR),
            13 => Ok(QuestionType::HINFO),
            14 => Ok(QuestionType::MINFO),
            15 => Ok(QuestionType::MX),
            16 => Ok(QuestionType::TXT),
            252 => Ok(QuestionType::AXFR),
            253 => Ok(QuestionType::MAILB),
            254 => Ok(QuestionType::MAILA),
            255 => Ok(QuestionType::ALL),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum QuestionClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    ANY = 255,
}

impl TryFrom<u16> for QuestionClass {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QuestionClass::IN),
            2 => Ok(QuestionClass::CS),
            3 => Ok(QuestionClass::CH),
            4 => Ok(QuestionClass::HS),
            255 => Ok(QuestionClass::ANY),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub question_type: QuestionType,
    pub question_class: QuestionClass,
}

impl From<Question> for Vec<u8> {
    fn from(val: Question) -> Self {
        let mut res = Vec::new();

        let name = encode_label(val.name);
        let question_type = (val.question_type as u16).to_be_bytes();
        let class = (val.question_class as u16).to_be_bytes();

        res.extend_from_slice(&name);
        res.extend_from_slice(&question_type);
        res.extend_from_slice(&class);

        res
    }
}

impl TryFrom<Vec<u8>> for Question {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let name: String = decode_label(value.clone());

        // skip both the end character of the name and the null terminating character
        let pointer = name.len() + 2;

        let question_type = (value[pointer] as u16) << 8 | value[pointer + 1] as u16;

        let question_type = QuestionType::try_from(question_type)?;
        let question_class = (value[pointer + 2] as u16) << 8 | value[pointer + 3] as u16;

        let question_class = QuestionClass::try_from(question_class)?;

        Ok(Question {
            name,
            question_type,
            question_class,
        })
    }
}

#[derive(Debug)]
pub enum ResourceType {
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

impl TryFrom<u16> for ResourceType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ResourceType::A),
            2 => Ok(ResourceType::NS),
            3 => Ok(ResourceType::MD),
            4 => Ok(ResourceType::MF),
            5 => Ok(ResourceType::CNAME),
            6 => Ok(ResourceType::SOA),
            7 => Ok(ResourceType::MB),
            8 => Ok(ResourceType::MG),
            9 => Ok(ResourceType::MR),
            10 => Ok(ResourceType::NULL),
            11 => Ok(ResourceType::WKS),
            12 => Ok(ResourceType::PTR),
            13 => Ok(ResourceType::HINFO),
            14 => Ok(ResourceType::MINFO),
            15 => Ok(ResourceType::MX),
            16 => Ok(ResourceType::TXT),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum ResourceClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl TryFrom<u16> for ResourceClass {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ResourceClass::IN),
            2 => Ok(ResourceClass::CS),
            3 => Ok(ResourceClass::CH),
            4 => Ok(ResourceClass::HS),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Answer {
    pub name: String,
    pub resource_type: ResourceType,
    pub class: ResourceClass,
    pub time_to_live: u32,
    pub length: u16,
    pub data: Vec<u8>,
}

impl From<Answer> for Vec<u8> {
    fn from(value: Answer) -> Self {
        let mut res: Vec<u8> = Vec::new();

        let name = encode_label(value.name);
        let resource_type = (value.resource_type as u16).to_be_bytes();
        let class = (value.class as u16).to_be_bytes();
        let time_to_live = value.time_to_live.to_be_bytes();
        let length = value.length.to_be_bytes();

        res.extend_from_slice(&name);
        res.extend_from_slice(&resource_type);
        res.extend_from_slice(&class);
        res.extend_from_slice(&time_to_live);
        res.extend_from_slice(&length);
        res.extend_from_slice(&value.data);

        res
    }
}

impl TryFrom<Vec<u8>> for Answer {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        println!("Value: {:?}", value);

        let name: String = decode_label(value.clone());

        println!("Name: {:?}", name);

        // skip both the end character of the name and the null terminating character
        let mut pointer = name.len() + 2;

        let resource_type = (value[pointer] as u16) << 8 | value[pointer + 1] as u16;
        let resource_type = ResourceType::try_from(resource_type)?;

        println!("Resource Type: {:?}", resource_type);

        let resource_class = (value[pointer + 2] as u16) << 8 | value[pointer + 3] as u16;
        let resource_class = ResourceClass::try_from(resource_class)?;

        println!("Resource Class: {:?}", resource_class);

        let time_to_live = (value[pointer + 4] as u32) << 24
            | (value[pointer + 5] as u32) << 16
            | (value[pointer + 6] as u32) << 8
            | value[pointer + 7] as u32;

        println!("Time to Live: {:?}", time_to_live);

        let length = (value[pointer + 8] as u16) << 8 | value[pointer + 9] as u16;

        println!("Length: {:?}", time_to_live);

        pointer += 10;
        let data = value[pointer..(pointer + length as usize)].to_vec();

        Ok(Answer {
            name,
            resource_type,
            class: resource_class,
            time_to_live,
            length,
            data,
        })
    }
}

fn encode_label(label: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for part in label.split(".") {
        res.push(part.len() as u8);
        res.extend(part.as_bytes());
    }

    res.push(0);

    res
}

fn decode_label(label: Vec<u8>) -> String {
    let mut label_parts = Vec::new();

    let mut pointer = 0;

    loop {
        let len = label[pointer] as usize;

        if len == 0 {
            break;
        }

        pointer += 1;

        let buf = &label[pointer..pointer + len];
        label_parts.push(String::from_utf8(buf.to_vec()).expect("Unable to decode label"));
        pointer = pointer + len;
    }

    label_parts.join(".")
}
