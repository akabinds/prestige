use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::{self, DirEntry},
    io,
    path::Path,
};

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET").expect("Target triple is not set");

    if target.contains("aarch64") {
        return Ok(());
    }

    visit_dirs(Path::new("src/kernel/arch"), &mut |entry| {
        let path = entry.path();

        let obj_file = path
            .file_name()
            .map(|s| s.to_str().unwrap())
            .expect("An issue occurred while getting the filename or converting it");

        match path.extension() {
            Some(ext) if ext.eq(&OsString::from("asm")) => {
                let mut build = nasm_rs::Build::new();

                build
                    .file(&path)
                    .flag("-felf64")
                    .compile(obj_file)
                    .expect("failed to compile assembly");

                println!("cargo:rustc-link-lib=static={obj_file}");
            }
            _ => (),
        }
    })?;

    Ok(())
}
