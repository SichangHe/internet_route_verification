use super::*;

fn count_sets_in_as_paths_in_all_ribs() {
    let rib_files = std::fs::read_dir("data/ribs")
        .unwrap()
        .map(|maybe_entry| maybe_entry.unwrap().path())
        .filter(|path| {
            path.is_file() && {
                let extension = path.extension().unwrap();
                extension == "gz" || extension == "bz2"
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(rib_files.len(), 59);

    let counts: Vec<_> = rib_files
        .par_iter()
        .map(|rib_file| {
            let rib_file_name = rib_file
                .file_name()
                .expect("RIB file should have a name.")
                .to_string_lossy();
            let collector = rib_file_name
                .split("--")
                .next()
                .expect("First split always succeeds.");

            let mut n_set = 0;
            let mut n_path_w_set = 0usize;
            let mut total = 0usize;

            let mut bgpdump_child = wrapper::read_mrt(rib_file).unwrap();
            let mut line = String::new();

            while bgpdump_child
                .stdout
                .read_line(&mut line)
                .expect("Error reading `bgpdump` output.")
                > 0
            {
                let line = Line::from_raw(std::mem::take(&mut line))
                    .expect("`bgpdump` should output valid lines.");
                let n_set_present = line
                    .compare
                    .as_path
                    .iter()
                    .filter(|a| !matches!(a, AsPathEntry::Seq(_)))
                    .count();
                if n_set_present > 0 {
                    n_set += n_set_present;
                    n_path_w_set += 1;
                }
                total += 1;
            }

            println!(
                "{collector}: {n_set} sets in the AS-path of {n_path_w_set} routes out of {total}."
            );
            (n_set, n_path_w_set, total)
        })
        .collect();

    let (n_set, n_path_w_set, total) = counts
        .iter()
        .copied()
        .reduce(|acc, (n_set, n_path_w_set, total)| {
            (acc.0 + n_set, acc.1 + n_path_w_set, acc.2 + total)
        })
        .unwrap();
    println!("Total: {n_set} sets in the AS-path of {n_path_w_set} routes out of {total}.");
}
