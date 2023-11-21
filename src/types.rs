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

pub enum OperationCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
}

pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
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
