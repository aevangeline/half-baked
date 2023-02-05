fn main() {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .compile(
            &["../shared_resources/protobufs/bread_service.proto"],
            &["../shared_resources/protobufs"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e))
}
