use std::net::UdpSocket;

struct DNSHeader {
    id: u16,
    qr_indicator: bool,
    op_code: u8,
    authoritative_answer: bool,
    truncation: bool,
    recursion_desired: bool,
    recursion_available: bool,
    response_code: u8,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_count: u16,
}

impl Into<[u8; 12]> for DNSHeader {
    fn into(self) -> [u8; 12] {
        let mut res = [0; 12];

        res[0] = self.id.to_be_bytes()[0];
        res[1] = self.id.to_be_bytes()[1];
        res[2] = (self.qr_indicator as u8) << 7
            | self.op_code << 3
            | (self.authoritative_answer as u8) << 2
            | (self.truncation as u8) << 1
            | (self.recursion_desired as u8);
        res[3] = (self.recursion_available as u8) << 7 | self.response_code;
        res[4] = self.question_count.to_be_bytes()[0];
        res[5] = self.question_count.to_be_bytes()[1];
        res[6] = self.answer_count.to_be_bytes()[0];
        res[7] = self.answer_count.to_be_bytes()[1];
        res[8] = self.authority_count.to_be_bytes()[0];
        res[9] = self.authority_count.to_be_bytes()[1];
        res[10] = self.additional_count.to_be_bytes()[0];
        res[11] = self.additional_count.to_be_bytes()[1];

        res
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, size);
                let response_header = DNSHeader {
                    id: 1234,
                    qr_indicator: true,
                    op_code: 0,
                    authoritative_answer: false,
                    truncation: false,
                    recursion_desired: false,
                    recursion_available: false,
                    response_code: 0,
                    question_count: 0,
                    answer_count: 0,
                    authority_count: 0,
                    additional_count: 0,
                };

                let response: [u8; 12] = response_header.into();

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
