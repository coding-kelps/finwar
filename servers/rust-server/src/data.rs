use polars::prelude::*;

pub fn read_csv(path: &str) -> Result<DataFrame, PolarsError> {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(
            CsvParseOptions::default().with_try_parse_dates(true),
        )
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

pub fn list_files_in_folder(path: &str) -> std::io::Result<Vec<String>> {
    use std::fs;
    use std::path::Path;
    let entries = fs::read_dir(Path::new(path))?;
    let files = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let file_type = e.file_type().ok()?;
                if file_type.is_file() {
                    Some(e.file_name().to_string_lossy().to_string())
                } else {
                    None
                }
            })
        })
        .collect();
    Ok(files)
}
