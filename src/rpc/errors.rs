error_chain! {
    errors {
        IO {
            description("Unexpected socket IO error")
            display("Unexpected socket IO error")
        }

        Complete {
            description("Client connection completed")
            display("Client connection completed")
        }

        BadPayload(s: &'static str) {
            description("Failed to decode payload")
                display("Bad payload: {}", s)
        }
    }
}
