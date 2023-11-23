# Basic DNS Server written in rust

This is a basic DNS server written in rust. It supports parsing DNS headers, question and answer sections, serialising DNS headers, questions and answers and setting a forwarding address using the `--resolver` flag

Created by following [codecrafter's guide](https://app.codecrafters.io/courses/dns-server/introduction) and reading [RFC 1034](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4)