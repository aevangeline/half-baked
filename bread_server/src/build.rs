fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../shared_resources/protobufs/rpc.proto"],
        &["../shared_resources/protobufs"],
    )?;
    Ok(())
}
