use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExerciseInfo {
    name: String,
    dir: String,
}

#[derive(Deserialize)]
struct InfoFile {
    exercises: Vec<ExerciseInfo>,
}

#[proc_macro]
pub fn include_files(_: TokenStream) -> TokenStream {
    let exercises = toml_edit::de::from_str::<InfoFile>(include_str!("../../info.toml"))
        .expect("Failed to parse `info.toml`")
        .exercises;

    let exercise_files = exercises
        .iter()
        .map(|exercise| format!("../exercises/{}/{}.rs", exercise.dir, exercise.name));
    let solution_files = exercises
        .iter()
        .map(|exercise| format!("../solutions/{}/{}.rs", exercise.dir, exercise.name));
    let dirs = exercises.iter().map(|exercise| &exercise.dir);
    let readmes = exercises
        .iter()
        .map(|exercise| format!("../exercises/{}/README.md", exercise.dir));

    quote! {
        EmbeddedFiles {
            exercise_files: &[#(ExerciseFiles { exercise: include_bytes!(#exercise_files), solution: include_bytes!(#solution_files) }),*],
            exercise_dirs: &[#(ExerciseDir { name: #dirs, readme: include_bytes!(#readmes) }),*]
        }
    }
    .into()
}
