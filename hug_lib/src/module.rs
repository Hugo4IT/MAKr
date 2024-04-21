use std::{
    env::current_dir,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{ident_table::IdentTable, variables::Variables};
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
    idents: IdentTable,
    variables: Variables,
    library: Option<Arc<Library>>,
}

impl HugModule {
    pub fn external(location: &str) -> Self {
        let library = unsafe { Library::new(resolve_library(location).unwrap()).unwrap() };

        let idents = IdentTable::new();
        let variables = Variables::new();

        Self {
            idents,
            variables,
            library: Some(Arc::new(library)),
        }
    }
}
