use super::*;

/// Write a CSV containing `last-modified` information for every object in
/// `input_dir`.
fn collect_last_modified(input_dirs: &[&str]) -> Result<()> {
    let start = Instant::now();
    let mut file = BufWriter::new(File::create("last_modified.csv")?);
    // Pipe-separated because `name` might contain `,`.
    file.write_all(b"class|name|source|last_modified\n")?;

    for input_dir in input_dirs {
        println!("Starining to scan {input_dir}.");
        for entry in read_dir(input_dir)? {
            let path = entry?.path();
            let reader = open_file_w_correct_encoding(&path)?;
            print!("|Starting to scan {path:?}.");

            for obj in rpsl_objects(io_wrapper_lines(reader)) {
                let mut last_modified = None;
                let mut source = None;
                for RpslExpr { key, expr } in expressions(lines_continued(obj.body.lines())) {
                    match key.as_str() {
                        "source" => source = Some(expr),
                        "last-modified" => last_modified = Some(expr),
                        _ => {}
                    }
                }
                let source_bytes = match &source {
                    Some(s) => s.as_bytes(),
                    None => b"null",
                };
                let last_modified_bytes = match &last_modified {
                    Some(s) => s.as_bytes(),
                    None => b"null",
                };

                file.write_all(obj.class.as_bytes())?;
                file.write_all(b"|")?;
                file.write_all(obj.name.as_bytes())?;
                file.write_all(b"|")?;
                file.write_all(source_bytes)?;
                file.write_all(b"|")?;
                file.write_all(last_modified_bytes)?;
                file.write_all(b"\n")?;
            }
            println!("|Scanned {path:?}.");
        }
    }

    file.flush()?;
    println!(
        "Scanned {input_dirs:?} in {}ms.",
        start.elapsed().as_millis()
    );
    Ok(())
}
