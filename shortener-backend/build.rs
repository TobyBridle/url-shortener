use std::{io::Result, path::Path};

fn main() -> Result<()> {
    let relative_path = Path::new("../protos/");
    // Convert relative path to absolute path
    match relative_path.canonicalize() {
        Ok(absolute_path) => {
            let mut paths = vec![];
            absolute_path
                .read_dir()?
                .filter(|f| {
                    f.as_ref()
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap()
                        .ends_with(".proto")
                })
                .for_each(|x| {
                    paths.push(x.unwrap().path().into_os_string().into_string().unwrap())
                });
            protobuf_codegen::Codegen::new()
                .protoc()
                .inputs(paths)
                .includes(&["src/", "models/", absolute_path.to_str().unwrap()])
                .out_dir("src/protos/")
                .run_from_script();
        }
        Err(err) => {
            eprintln!("Error: {} at {:?}", err, relative_path);
        }
    }
    Ok(())
}
