use std::{
    fs::{create_dir_all, read_dir, File},
    io::{BufReader, Write},
    path::Path,
    thread::available_parallelism,
};

use io::serialize::from_reader;
use itertools::izip;

use super::*;

/// Parsed RPSL intermediate representation.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Ir {
    pub aut_nums: BTreeMap<u32, AutNum>,
    pub as_sets: BTreeMap<String, AsSet>,
    pub route_sets: BTreeMap<String, RouteSet>,
    pub peering_sets: BTreeMap<String, PeeringSet>,
    pub filter_sets: BTreeMap<String, FilterSet>,
    /// The AS numbers with Vec of their routes.
    /// <https://www.rfc-editor.org/rfc/rfc2622#section-4>.
    /// Each value should always be sorted.
    pub as_routes: BTreeMap<u32, Vec<IpNet>>,
}

pub fn split_n_btreemap<K, V>(mut map: BTreeMap<K, V>, n: usize) -> Vec<BTreeMap<K, V>>
where
    K: std::cmp::Ord + Clone,
{
    let length = map.len();
    if length < n {
        return split_n_btreemap(map, length);
    }

    let size_per_split = map.len() / n - 1;
    let mut splits = Vec::with_capacity(n);
    for _ in 0..(n - 1) {
        let split_point = map.iter().rev().nth(size_per_split).unwrap().0.clone();
        let split = map.split_off(&split_point);
        splits.push(split);
    }
    splits.push(map);
    splits
}

impl Ir {
    pub fn split_n(self, n: usize) -> Vec<Self> {
        let Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        } = self;
        let aut_num_splits = split_n_btreemap(aut_nums, n);
        let as_set_splits = split_n_btreemap(as_sets, n);
        let route_set_splits = split_n_btreemap(route_sets, n);
        let peering_set_splits = split_n_btreemap(peering_sets, n);
        let filter_set_splits = split_n_btreemap(filter_sets, n);
        let as_route_splits = split_n_btreemap(as_routes, n);

        izip!(
            aut_num_splits,
            as_set_splits,
            route_set_splits,
            peering_set_splits,
            filter_set_splits,
            as_route_splits
        )
        .map(
            |(aut_nums, as_sets, route_sets, peering_sets, filter_sets, as_routes)| Self {
                aut_nums,
                as_sets,
                route_sets,
                peering_sets,
                filter_sets,
                as_routes,
            },
        )
        .collect()
    }

    /// Split `self` based on the number of CPU logic cores available × 4.
    pub fn split_n_cpus(self) -> Result<Vec<Self>> {
        let n: usize = available_parallelism()?.into();
        Ok(self.split_n(n * 4))
    }

    /// Split `self` and write to `directory` in parallel.
    /// Non-existent `directory` is automatically created;
    /// otherwise, it is assumed empty.
    pub fn pal_write<P>(self, directory: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let splits = self.split_n_cpus()?;
        pal_write_ir(&splits, directory)
    }

    /// When both [`Ir`]s have the same keys, choose `other`'s value.
    pub fn merge(mut self, other: Self) -> Self {
        let Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        } = other;
        self.aut_nums.extend(aut_nums);
        self.as_sets.extend(as_sets);
        self.route_sets.extend(route_sets);
        self.peering_sets.extend(peering_sets);
        self.filter_sets.extend(filter_sets);
        self.as_routes.extend(as_routes);
        self
    }

    /// Read a [`Ir`] from `directory` in parallel.
    /// All files need to JSON and serialized from [`Ir`],
    /// presumably, they were written using [`pal_write`](#method.pal_write)
    /// in the first place.
    /// No guarantee about the priorities of the files.
    pub fn pal_read<P>(directory: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let readers = read_dir(directory)?
            .map(|p| Ok(BufReader::new(File::open(p?.path())?)))
            .collect::<Result<Vec<_>>>()?;
        let irs = readers
            .into_par_iter()
            .map(from_reader)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(merge_irs(irs))
    }
}

impl std::fmt::Display for Ir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            aut_nums,
            as_sets,
            route_sets,
            peering_sets,
            filter_sets,
            as_routes,
        } = self;
        f.write_fmt(format_args!(
            "{} aut_nums, {} as_sets, {} route_sets, {} peering_sets, {} filter_sets, {} as_routes",
            aut_nums.len(),
            as_sets.len(),
            route_sets.len(),
            peering_sets.len(),
            filter_sets.len(),
            as_routes.len(),
        ))
    }
}

pub fn pal_write_ir<P>(splits: &Vec<Ir>, directory: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let directory = directory.as_ref().to_owned();
    create_dir_all(&directory)?;
    let writes = splits
        .par_iter()
        .enumerate()
        .map(|(index, ir)| {
            let path = directory.clone().join(format!("{index}.json"));
            let file = File::create(path)?;
            let json = serde_json::to_string(ir)?;
            Ok((file, json))
        })
        .collect::<Result<Vec<_>>>()?;
    for (mut file, json) in writes {
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}

/// Merge `irs` into a single [`Ir`] in parallel, with no ordering guarantee.
pub fn merge_irs<I>(irs: I) -> Ir
where
    I: IntoParallelIterator<Item = Ir>,
{
    irs.into_par_iter().reduce(Ir::default, Ir::merge)
}
