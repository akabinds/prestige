use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let kernel_name = env::var("CARGO_PKG_NAME")?;

    println!("cargo:rustc-link-arg-bin={kernel_name}=--script=linker.ld");

    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");

    Ok(())
}
