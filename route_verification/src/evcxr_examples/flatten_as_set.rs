use super::*;

/// Fully flatten each AS Set to all of its members.
/// Copy this after running code from [`parse_bgp_lines`].
fn flatten_as_sets(query: QueryIr) -> Result<()> {
    fn flatten(
        as_set: &mut HashSet<u32>,
        visited_sets: &mut HashSet<String>,
        set_members: &[String],
        as_sets: &HashMap<String, AsSet>,
    ) -> usize {
        let mut max_depth = 0;

        for set_member in set_members {
            if !visited_sets.contains(set_member) {
                visited_sets.insert(set_member.to_string());
                if let Some(set) = as_sets.get(set_member) {
                    as_set.extend(set.members.iter().copied());

                    let depth = 1 + flatten(as_set, visited_sets, &set.set_members, as_sets);
                    max_depth = max_depth.max(depth);
                }
            }
        }

        max_depth
    }

    let start = Instant::now();
    let as_sets: HashMap<String, (HashSet<u32>, usize)> = query
        .as_sets
        .par_iter()
        .map(|(name, set)| {
            let mut members: HashSet<u32> =
                HashSet::with_capacity(set.set_members.len() * 32 + set.members.len());
            members.extend(set.members.iter().copied());

            let mut visited = HashSet::with_capacity(set.set_members.len() * 8);
            visited.insert(name.to_string());
            let depth = flatten(&mut members, &mut visited, &set.set_members, &query.as_sets);

            (name.to_owned(), (members, depth))
        })
        .collect();
    println!(
        "Flattened {} AS Sets in {}ms.",
        as_sets.len(),
        start.elapsed().as_millis()
    );

    {
        let mut as_set_file = BufWriter::new(File::create("as_sets.txt")?);
        for (num, (as_set, _depth)) in &as_sets {
            as_set_file.write_all(num.to_string().as_bytes());
            as_set_file.write_all(b";");
            for (index, member) in as_set.iter().enumerate() {
                if index > 0 {
                    as_set_file.write_all(b",");
                }
                as_set_file.write_all(member.to_string().as_bytes());
            }
            as_set_file.write_all(b"\n");
        }
        as_set_file.flush()?;
    }

    {
        let mut as_set_sizes_file = BufWriter::new(File::create("as_set_sizes.csv")?);
        as_set_sizes_file.write_all(b"as_set,size,depth\n")?;
        for (num, (as_set, depth)) in &as_sets {
            as_set_sizes_file.write_all(num.to_string().as_bytes());
            as_set_sizes_file.write_all(b",");
            as_set_sizes_file.write_all(as_set.len().to_string().as_bytes());
            as_set_sizes_file.write_all(b",");
            as_set_sizes_file.write_all(depth.to_string().as_bytes());
            as_set_sizes_file.write_all(b"\n");
        }
        as_set_sizes_file.flush()?;
    }

    Ok(())
}
