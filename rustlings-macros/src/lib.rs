use proc_macro::TokenStream;
use quote::quote;
use std::{fs::read_dir, panic, path::PathBuf};

fn path_to_string(path: PathBuf) -> String {
    path.into_os_string()
        .into_string()
        .unwrap_or_else(|original| {
            panic!("The path {} is invalid UTF8", original.to_string_lossy());
        })
}

#[proc_macro]
pub fn include_files(_: TokenStream) -> TokenStream {
    let mut files = Vec::with_capacity(8);
    let mut dirs = Vec::with_capacity(128);

    for entry in read_dir("exercises").expect("Failed to open the exercises directory") {
        let entry = entry.expect("Failed to read the exercises directory");

        if entry.file_type().unwrap().is_file() {
            let path = entry.path();
            if path.file_name().unwrap() != "README.md" {
                files.push(path_to_string(path));
            }

            continue;
        }

        let dir_path = entry.path();
        let dir_files = read_dir(&dir_path).unwrap_or_else(|e| {
            panic!("Failed to open the directory {}: {e}", dir_path.display());
        });
        let dir_path = path_to_string(dir_path);
        let dir_files = dir_files.filter_map(|entry| {
            let entry = entry.unwrap_or_else(|e| {
                panic!("Failed to read the directory {dir_path}: {e}");
            });
            let path = entry.path();

            if !entry.file_type().unwrap().is_file() {
                panic!("Found {} but expected only files", path.display());
            }

            if path.file_name().unwrap() == "README.md" {
                return None;
            }

            if path.extension() != Some("rs".as_ref()) {
                panic!(
                    "Found {} but expected only README.md and .rs files",
                    path.display(),
                );
            }

            Some(path_to_string(path))
        });

        dirs.push(quote! {
            EmbeddedFlatDir {
                path: #dir_path,
                readme: EmbeddedFile {
                    path: concat!(#dir_path, "/README.md"),
                    content: ::std::include_bytes!(concat!("../", #dir_path, "/README.md")),
                },
                content: &[
                    #(EmbeddedFile {
                        path: #dir_files,
                        content: ::std::include_bytes!(concat!("../", #dir_files)),
                    }),*
                ],
            }
        });
    }

    quote! {
        EmbeddedFiles {
            info_toml_content: ::std::include_str!("../info.toml"),
            exercises_dir: ExercisesDir {
                readme: EmbeddedFile {
                    path: "exercises/README.md",
                    content: ::std::include_bytes!("../exercises/README.md"),
                },
                files: &[#(
                     EmbeddedFile {
                        path: #files,
                        content: ::std::include_bytes!(concat!("../", #files)),
                    }
                ),*],
                dirs: &[#(#dirs),*],
            },
        }
    }
    .into()
}
