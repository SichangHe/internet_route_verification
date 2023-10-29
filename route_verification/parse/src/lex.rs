use ::lex::{self, Counts};
use lazy_regex::regex_captures;

use super::*;

pub fn parse_lexed(lexed: lex::Ast) -> (Ir, Counts) {
    debug!("Start to parse lexed AST.");
    let lex::Ast {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
        filter_sets,
        as_routes,
    } = lexed;
    let (aut_nums, an_counts) = parse_lexed_aut_nums(aut_nums);
    let (as_sets, as_counts) = parse_lexed_as_sets(as_sets);
    let (route_sets, rs_counts) = parse_lexed_route_sets(route_sets);
    let (peering_sets, ps_counts) = parse_lexed_peering_sets(peering_sets);
    let (filter_sets, fs_counts) = parse_lexed_filter_sets(filter_sets);
    let (as_routes, ar_counts) = parse_lexed_as_routes(as_routes);
    let ir = Ir {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
        filter_sets,
        as_routes,
    };
    debug!("parse_lexed: Parsed {ir}.");
    let counts = an_counts + as_counts + rs_counts + ps_counts + fs_counts + ar_counts;
    (ir, counts)
}

pub fn parse_lexed_aut_nums(lexed: Vec<lex::AutNum>) -> (BTreeMap<u32, AutNum>, Counts) {
    par_process_map_counts(
        lexed,
        |(mut acc, mut counts), lexed| match parse_lexed_aut_num(lexed, &mut counts) {
            Ok((num, aut_num)) => {
                acc.insert(num, aut_num);
                (acc, counts)
            }
            Err(e) => {
                counts.parse_aut_num += 1;
                error!("{e:?}");
                (acc, counts)
            }
        },
    )
}

pub fn parse_lexed_aut_num(aut_num: lex::AutNum, counts: &mut Counts) -> Result<(u32, AutNum)> {
    let num = parse_aut_num_name(&aut_num.name).context(format!("parsing {aut_num:?}"))?;
    let lex::AutNum {
        name: _,
        body,
        imports,
        exports,
    } = aut_num;
    let imports = parse_imports(imports, counts);
    let exports = parse_imports(exports, counts);
    Ok((
        num,
        AutNum {
            body,
            imports,
            exports,
        },
    ))
}

pub fn parse_aut_num_name(name: &str) -> Result<u32> {
    match regex_captures!(r"^AS(\d+)$"i, name) {
        Some((_, num)) => num
            .parse()
            .map_err(|err| Error::new(err).context(format!("parsing {name}"))),
        None => bail!("AS number name `{name}` does not match pattern"),
    }
}

pub fn parse_lexed_as_sets(lexed: Vec<lex::AsOrRouteSet>) -> (BTreeMap<String, AsSet>, Counts) {
    par_process_map_counts(
        lexed,
        |(mut acc, mut counts), lexed| match parse_lexed_as_set(lexed) {
            Ok((name, as_set)) => {
                acc.insert(name, as_set);
                (acc, counts)
            }
            Err(e) => {
                counts.parse_as_set += 1;
                error!("{e:?}");
                (acc, counts)
            }
        },
    )
}

pub fn parse_lexed_as_set(lexed: lex::AsOrRouteSet) -> Result<(String, AsSet)> {
    if !is_as_set(&lexed.name) && !is_pseudo_set(&lexed.name) {
        bail!("invalid AS Set name in {lexed:?}");
    }
    let max_length = lexed.members.len();
    let mut members = Vec::with_capacity(max_length);
    let mut set_members = Vec::with_capacity(max_length);
    for member in lexed.members {
        let member = parse_as_name(member)
            .with_context(|| format!("parsing AS Set {}\n{}", lexed.name, lexed.body))?;
        match member {
            AsName::Any => return Ok((lexed.name, AsSet::new_any(lexed.body))),
            AsName::Num(n) => members.push(n),
            AsName::Set(set) => set_members.push(set),
            AsName::Invalid(reason) => {
                bail!("{reason} parsing AS Set {}\n{}", lexed.name, lexed.body)
            }
        }
    }
    let as_set = AsSet::new(lexed.body, members, set_members);
    Ok((lexed.name, as_set))
}

pub fn parse_lexed_route_sets(
    lexed: Vec<lex::AsOrRouteSet>,
) -> (BTreeMap<String, RouteSet>, Counts) {
    par_process_map_counts(
        lexed,
        |(mut acc, mut counts), lexed| match parse_lexed_route_set(lexed) {
            Ok((name, route_set)) => {
                acc.insert(name, route_set);
                (acc, counts)
            }
            Err(e) => {
                counts.parse_route_set += 1;
                error!("{e:?}");
                (acc, counts)
            }
        },
    )
}

pub fn parse_lexed_route_set(lexed: lex::AsOrRouteSet) -> Result<(String, RouteSet)> {
    if !is_route_set_name(&lexed.name) && !is_pseudo_set(&lexed.name) {
        bail!(
            "{} is an invalid route set name—parsing {lexed:?}",
            lexed.name
        );
    }
    let members = lexed
        .members
        .into_iter()
        .map(|member| member.into())
        .collect();

    Ok((
        lexed.name,
        RouteSet {
            body: lexed.body,
            members,
        },
    ))
}

pub fn parse_lexed_peering_sets(
    lexed: Vec<lex::PeeringSet>,
) -> (BTreeMap<String, PeeringSet>, Counts) {
    par_process_map_counts(
        lexed,
        |(mut acc, mut counts), lexed| match parse_lexed_peering_set(lexed) {
            Ok((name, peering_set)) => {
                acc.insert(name, peering_set);
                (acc, counts)
            }
            Err(e) => {
                counts.parse_peering_set += 1;
                error!("{e:?}");
                (acc, counts)
            }
        },
    )
}

pub fn parse_lexed_peering_set(lexed: lex::PeeringSet) -> Result<(String, PeeringSet)> {
    if !is_peering_set(&lexed.name) {
        bail!(
            "{} is an invalid peering set name—parsing {lexed:?}",
            lexed.name
        );
    }
    Ok((
        lexed.name,
        PeeringSet {
            body: lexed.body,
            peerings: lexed.peerings.into_iter().map(parse_mp_peering).collect(),
        },
    ))
}

pub fn parse_lexed_filter_sets(
    lexed: Vec<lex::FilterSet>,
) -> (BTreeMap<String, FilterSet>, Counts) {
    par_process_map_counts(
        lexed,
        |(mut acc, mut counts), lexed| match parse_lexed_filter_set(lexed, &mut counts) {
            Ok((name, filter_set)) => {
                acc.insert(name, filter_set);
                (acc, counts)
            }
            Err(e) => {
                counts.parse_filter_set += 1;
                error!("{e:?}");
                (acc, counts)
            }
        },
    )
}

pub fn parse_lexed_filter_set(
    lexed: lex::FilterSet,
    counts: &mut Counts,
) -> Result<(String, FilterSet)> {
    if !is_filter_set(&lexed.name) {
        bail!(
            "{} is an invalid filter set name—parsing {lexed:?}",
            lexed.name
        );
    }
    let filter_set = FilterSet {
        body: lexed.body,
        filters: lexed
            .filters
            .into_iter()
            .map(|f| parse_filter(f, &[], counts))
            .collect(),
    };
    Ok((lexed.name, filter_set))
}

pub fn parse_lexed_as_routes(
    as_routes: BTreeMap<String, Vec<String>>,
) -> (BTreeMap<u32, Vec<IpNet>>, Counts) {
    par_process_map_counts(
        as_routes,
        |(mut acc, mut counts), lexed| match parse_lexed_as_route(&lexed) {
            Ok((num, routes)) => {
                acc.insert(num, routes);
                (acc, counts)
            }
            Err(e) => {
                counts.parse_as_route += 1;
                error!("Parsing routes for {lexed:?}: {e}.");
                (acc, counts)
            }
        },
    )
}

pub fn par_process_map_counts<I, In, F, K, V>(input: I, transform: F) -> (BTreeMap<K, V>, Counts)
where
    I: IntoParallelIterator<Item = In>,
    F: Fn((BTreeMap<K, V>, Counts), In) -> (BTreeMap<K, V>, Counts) + Send + Sync,
    K: Ord + Send + Sync,
    V: Send + Sync,
{
    input
        .into_par_iter()
        .fold(|| (BTreeMap::new(), Counts::default()), transform)
        .reduce(
            || (BTreeMap::new(), Counts::default()),
            |(mut map0, counts0), (map1, counts1)| {
                map0.extend(map1);
                (map0, counts0 + counts1)
            },
        )
}

pub fn parse_lexed_as_route((name, routes): &(String, Vec<String>)) -> Result<(u32, Vec<IpNet>)> {
    let num = parse_aut_num_name(name)?;
    let routes: Result<_> = routes.iter().map(|r| Ok(r.parse()?)).collect();
    let mut routes: Vec<_> = routes?;
    routes.sort_unstable();
    Ok((num, routes))
}

pub fn is_pseudo_set(s: &str) -> bool {
    matches!(s.as_bytes().get(1), Some(&b'#'))
}
