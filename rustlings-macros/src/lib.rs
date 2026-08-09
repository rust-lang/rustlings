use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExerciseInfo<'a> {
    name: &'a str,
    dir: &'a str,
    #[serde(default)]
    input_files: Vec<&'a str>,
}

#[derive(Deserialize)]
struct InfoFile<'a> {
    #[serde(borrow)]
    exercises: Vec<ExerciseInfo<'a>>,
}

#[proc_macro]
pub fn include_files(_: TokenStream) -> TokenStream {
    let info_file = include_str!("../info.toml");
    let info = toml::de::from_str::<InfoFile>(info_file).expect("Failed to parse `info.toml`");
    let exercises = info.exercises;

    let exercise_files = exercises
        .iter()
        .map(|exercise| format!("../exercises/{}/{}.rs", exercise.dir, exercise.name));
    let solution_files = exercises
        .iter()
        .map(|exercise| format!("../solutions/{}/{}.rs", exercise.dir, exercise.name));

    let mut dirs = Vec::with_capacity(32);
    let mut dir_inds = vec![0; exercises.len()];

    for (exercise, dir_ind) in exercises.iter().zip(&mut dir_inds) {
        // The directory is often the last one inserted.
        if let Some(ind) = dirs.iter().rev().position(|dir| *dir == exercise.dir) {
            *dir_ind = dirs.len() - 1 - ind;
            continue;
        }

        dirs.push(exercise.dir);
        *dir_ind = dirs.len() - 1;
    }

    let input_files = exercises.iter().map(|exercise| {
        let names = exercise.input_files.iter();
        let paths = exercise
            .input_files
            .iter()
            .map(|f| format!("../exercises/{}/{}", exercise.dir, f));
        quote! {
            &[#(InputFile {
                name: #names,
                content: include_str!(#paths),
            }),*]
        }
    });

    let readmes = dirs
        .iter()
        .map(|dir| format!("../exercises/{dir}/README.md"));

    quote! {
        EmbeddedFiles {
            info_file: #info_file,
            exercise_files: &[#(ExerciseFiles {
                exercise: include_bytes!(#exercise_files),
                solution: include_bytes!(#solution_files),
                dir_ind: #dir_inds,
                input_files: #input_files,
            }),*],
            exercise_dirs: &[#(ExerciseDir { name: #dirs, readme: include_bytes!(#readmes) }),*],
        }
    }
    .into()
}
