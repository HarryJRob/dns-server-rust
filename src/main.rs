use std::net::UdpSocket;

use crate::types::{
    Answer, Header, Message, OperationCode, Question, QuestionClass, QuestionType, ResourceClass,
    ResourceType, ResponseCode,
};

mod types;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let received_message = Message::try_from(buf.to_vec());

                println!("1. Received Message: {:?}", received_message);

                let response_message = if let Ok(received_message) = received_message {
                    Message {
                        header: Header {
                            id: received_message.header.id,
                            qr_indicator: true,
                            op_code: received_message.header.op_code,
                            authoritative_answer: false,
                            truncation: false,
                            recursion_desired: received_message.header.recursion_desired,
                            recursion_available: false,
                            response_code: match received_message.header.op_code {
                                OperationCode::Query => ResponseCode::NoError,
                                _ => ResponseCode::NotImplemented,
                            },
                            question_count: 1,
                            answer_count: 1,
                            authority_count: 0,
                            additional_count: 0,
                        },
                        questions: received_message.questions,
                        answers: received_message.answers,
                    }
                } else {
                    Message {
                        header: Header {
                            id: 1234,
                            qr_indicator: true,
                            op_code: OperationCode::Query,
                            authoritative_answer: false,
                            truncation: false,
                            recursion_desired: false,
                            recursion_available: false,
                            response_code: ResponseCode::FormatError,
                            question_count: 1,
                            answer_count: 1,
                            authority_count: 0,
                            additional_count: 0,
                        },
                        questions: vec![Question {
                            name: "codecrafters.io".to_string(),
                            question_type: QuestionType::A,
                            question_class: QuestionClass::IN,
                        }],
                        answers: vec![Answer {
                            name: "codecrafters.io".to_string(),
                            resource_type: ResourceType::A,
                            class: ResourceClass::IN,
                            time_to_live: 60,
                            length: 4,
                            data: vec![8, 8, 8, 8],
                        }],
                    }
                };

                println!("2. Response Message: {:?}", response_message);

                let response: Vec<u8> = response_message.into();

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
