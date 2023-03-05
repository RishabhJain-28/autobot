use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir);
    let runtime_dest_path = dest_path.join("runtime");

    copy_recursively("./src/runtime", &runtime_dest_path).unwrap();
    println!("cargo:rerun-if-changed=/src/runtime");
    // copy_cargo("./", &dest_path).expect("Cannot copy dependency: cargo.toml");
}

pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

// pub fn copy_cargo(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
//     fs::create_dir_all(&destination)?;
//     let mut dir = fs::read_dir(source)?;
//     let file = dir.find(|entry| {
//         entry
//             .as_ref()
//             .unwrap()
//             .file_name()
//             .into_string()
//             .unwrap()
//             .contains("Cargo.toml")
//     });
//     let file = file.unwrap().unwrap();
//     fs::copy(file.path(), destination.as_ref().join(file.file_name()))?;
//     // for entry in dir {
//     //     let entry = entry?;
//     //     let filetype = entry.file_type()?;
//     //     if filetype.is_file()
//     //         && entry
//     //             .file_name()
//     //             .into_string()
//     //             .unwrap()
//     //             .contains("Cargo.toml")
//     //     {
//     //         fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
//     //     };
//     // }
//     Ok(())
// }
