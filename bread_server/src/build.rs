fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "../shared_resources/protobufs/users.proto",
            "../shared_resources/protobufs/backend.proto",
        ],
        &["../shared_resources/protobufs"],
    )?;
    Ok(())
}
