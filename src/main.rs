use std::{env::args, net::UdpSocket};

use crate::types::{
    Answer, DomainName, Header, Message, OperationCode, Question, QuestionClass, QuestionType,
    ResourceClass, ResourceType, ResponseCode,
};

mod types;

fn main() {
    let receiver_socket =
        UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind receiver socket");

    let resolver_socket =
        if let Some(addr) = args().skip_while(|arg| arg != "--resolver").skip(1).next() {
            println!("Attempting to bind to resolver... {:?}", addr);

            let resolver_socket =
                UdpSocket::bind("127.0.0.1:2054").expect("Failed to bind resolver socket");

            resolver_socket
                .connect(addr)
                .expect("Unable to connect to the resolver server");

            Some(resolver_socket)
        } else {
            None
        };

    let mut buf = [0; 512];

    loop {
        match receiver_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let received_message = Message::try_from(buf.to_vec());

                println!("1. Received Message: {:?}", received_message);

                let response_message = if let Ok(received_message) = received_message {
                    let mut answers = Vec::new();

                    for question in &received_message.questions {
                        match &resolver_socket {
                            Some(resolver_socket) => {
                                println!("Resolving using resolver socket");

                                let resolver_message = Message {
                                    header: Header {
                                        id: rand::random(),
                                        qr_indicator: received_message.header.qr_indicator,
                                        op_code: received_message.header.op_code,
                                        authoritative_answer: false,
                                        truncation: false,
                                        recursion_desired: false,
                                        recursion_available: false,
                                        response_code: ResponseCode::NoError,
                                        question_count: 1,
                                        answer_count: 0,
                                        authority_count: 0,
                                        additional_count: 0,
                                    },
                                    questions: vec![question.clone()],
                                    answers: vec![],
                                };

                                let resolver_message: Vec<u8> = resolver_message.into();

                                resolver_socket
                                    .send(&resolver_message)
                                    .expect("Unknown error when trying to forward request");

                                resolver_socket
                                    .recv_from(&mut buf)
                                    .expect("No response from resolver socket");

                                let resolver_response = Message::try_from(buf.to_vec())
                                    .expect("Unable to parse resolver response");

                                println!("Resolver Response: {:?}", resolver_response);

                                answers.extend(resolver_response.answers);
                            }
                            None => {
                                answers.push(Answer {
                                    name: question.name.clone(),
                                    resource_type: ResourceType::A,
                                    class: ResourceClass::IN,
                                    time_to_live: 60,
                                    length: 4,
                                    data: vec![8, 8, 8, 8],
                                });
                            }
                        }
                    }

                    let answers = answers;

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
                            question_count: received_message.questions.len() as u16,
                            answer_count: answers.len() as u16,
                            authority_count: 0,
                            additional_count: 0,
                        },
                        questions: received_message.questions,
                        answers,
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
                            name: DomainName::new("codecrafters.io".to_string()),
                            question_type: QuestionType::A,
                            question_class: QuestionClass::IN,
                        }],
                        answers: vec![Answer {
                            name: DomainName::new("codecrafters.io".to_string()),
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

                receiver_socket
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
