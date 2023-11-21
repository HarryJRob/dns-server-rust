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
                let received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, size);

                let message = Message {
                    header: Header {
                        id: 1234,
                        qr_indicator: true,
                        op_code: OperationCode::Query,
                        authoritative_answer: false,
                        truncation: false,
                        recursion_desired: false,
                        recursion_available: false,
                        response_code: ResponseCode::NoError,
                        question_count: 1,
                        answer_count: 1,
                        authority_count: 0,
                        additional_count: 0,
                    },
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
                        length: 4,
                        data: vec![8, 8, 8, 8],
                    }],
                };

                let response: Vec<u8> = message.into();

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
