fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("src/strategy/out_of_process/out_of_process.proto")?;
    Ok(())
}
