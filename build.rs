use std::{env, fs::File, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/core.proto")?;
    File::create(env::var("OUT_DIR").expect("NO OUTPUT").to_string() + "/target")?
        .write(env::var("TARGET").expect("NO TARGET").as_bytes())?;
    Ok(())
}
