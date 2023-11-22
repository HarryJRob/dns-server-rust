pub struct Message {
    pub header: Header,
    pub question: Question,
    pub answers: Vec<Answer>,
}

impl From<Message> for Vec<u8> {
    fn from(val: Message) -> Self {
        let mut res = Vec::new();

        let header: [u8; 12] = val.header.into();
        let question: Vec<u8> = val.question.into();

        res.extend_from_slice(&header);
        res.extend_from_slice(&question);

        for resource in val.answers {
            let resource: Vec<u8> = resource.into();

            res.extend_from_slice(&resource);
        }

        res
    }
}

impl From<Vec<u8>> for Message {
    fn from(value: Vec<u8>) -> Self {
        if value.len() < 12 {
            panic!("Unable to parse header")
        };

        let header = &value[0..12];
        let mut header_slice: [u8; 12] = [0; 12];
        header_slice.clone_from_slice(header);
        let header = Header::from(header_slice);

        Message {
            header,
            question: Question {
                name: "codecrafters.io".to_string(),
                question_type: QuestionType::A,
                class: QuestionClass::IN,
            },
            answers: vec![Answer {
                name: "codecrafters.io".to_string(),
                resource_type: ResourceType::A,
                class: ResourceClass::IN,
                time_to_live: 60,
                length: 0,
                data: vec![],
            }],
        }
    }
}

#[derive(Clone, Copy)]
pub enum OperationCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
}

impl From<u8> for OperationCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OperationCode::Query,
            1 => OperationCode::IQuery,
            2 => OperationCode::Status,
            _ => panic!("Unknown operation code"),
        }
    }
}

pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl From<u8> for ResponseCode {
    fn from(value: u8) -> Self {
        match value {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFailure,
            3 => ResponseCode::NameError,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            _ => panic!("Unknown response code"),
        }
    }
}

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

impl From<[u8; 12]> for Header {
    fn from(value: [u8; 12]) -> Self {
        let id: u16 = (value[0] as u16) << 8 | (value[1] as u16);
        let qr_indicator = (value[2] >> 7 & 1) == 1;

        let op_code = OperationCode::from(value[2] >> 3 & 0b00001111);
        let authoritative_answer = (value[2] >> 2 & 1) == 1;
        let truncation = (value[2] >> 1 & 1) == 1;
        let recursion_desired = (value[2] & 1) == 1;
        let recursion_available = (value[3] >> 7 & 1) == 1;
        let response_code = ResponseCode::from(value[3] & 0b00001111);
        let question_count = (value[4] as u16) << 8 | (value[5] as u16);
        let answer_count = (value[6] as u16) << 8 | (value[7] as u16);
        let authority_count = (value[8] as u16) << 8 | (value[9] as u16);
        let additional_count = (value[10] as u16) << 8 | (value[11] as u16);

        Header {
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
        }
    }
}

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

pub enum QuestionClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    ANY = 255,
}

pub struct Question {
    pub name: String,
    pub question_type: QuestionType,
    pub class: QuestionClass,
}

impl From<Question> for Vec<u8> {
    fn from(val: Question) -> Self {
        let mut res = Vec::new();

        let name = encode_string(val.name);
        let question_type = (val.question_type as u16).to_be_bytes();
        let class = (val.class as u16).to_be_bytes();

        res.extend_from_slice(&name);
        res.extend_from_slice(&question_type);
        res.extend_from_slice(&class);

        res
    }
}

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

pub enum ResourceClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

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

        let name = encode_string(value.name);
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

fn encode_string(string: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    for part in string.split(".") {
        res.push(part.len() as u8);
        res.extend(part.as_bytes());
    }

    res.push(0);

    res
}
