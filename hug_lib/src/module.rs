use std::{
    env::current_dir,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    function::HugFunction,
    ident_table::IdentTable,
    value::{HugExternalFunction, HugValue},
    variables::Variables,
    Ident,
};
use libloading::Library;

fn target_dir() -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    const BUILD_TYPE: &str = "debug";
    #[cfg(not(debug_assertions))]
    const BUILD_TYPE: &str = "release";

    std::env::var("CARGO_TARGET_DIR")
        .ok()
        .map(|s| Path::new(&s).to_path_buf())
        .or_else(|| current_dir().ok().map(|dir| dir.join("target")))
        .map(|path| path.join(BUILD_TYPE))
}

fn resolve_library(path: &str) -> Option<PathBuf> {
    let file_name = libloading::library_filename(path);

    if let Some(target_dir) = target_dir() {
        let path = target_dir.join(file_name);
        if path.exists() {
            return Some(path);
        }
    } else {
        eprintln!("Warning: cargo target dir was not found");
    }

    None
}

#[derive(Debug, Clone)]
pub struct HugModule {
    variables: Variables,
    library: Option<Arc<Library>>,
}

impl HugModule {
    pub fn external(location: &str) -> Self {
        let library = unsafe { Library::new(resolve_library(location).unwrap()).unwrap() };

        let variables = Variables::new();

        Self {
            variables,
            library: Some(Arc::new(library)),
        }
    }

    pub fn import(&mut self, ident_table: &IdentTable, path: &[Ident]) -> HugValue {
        if path.is_empty() {
            panic!("Invalid import");
        }

        if path.len() != 1 {
            match self.variables.get_mut(path[0]) {
                Some(HugValue::Module(module)) => return module.import(ident_table, &path[1..]),
                _ => panic!("Invalid import"),
            }
        }

        let ident = path[0];

        if let Some(variable) = self.variables.get(ident) {
            eprintln!("warning: duplicate import.");

            return variable.clone();
        }

        if let Some(library) = self.library.as_ref() {
            let symbol: libloading::Symbol<HugExternalFunction> = unsafe {
                library
                    .get(format!("_HUG_EXPORT_{}", ident_table.name(ident)).as_bytes())
                    .unwrap()
            };

            let function = HugValue::Function(HugFunction::External {
                function_pointer: *symbol,
            });

            self.variables.set(ident, function.to_owned());

            function
        } else {
            todo!()
        }
    }
}
