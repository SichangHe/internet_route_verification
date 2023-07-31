use ::lex;
use lazy_regex::regex_captures;

use super::*;

pub fn parse_lexed(lexed: lex::Dump) -> Dump {
    debug!("Start to parse lexed dump.");
    let lex::Dump {
        aut_nums,
        as_sets,
        route_sets,
        peering_sets,
        filter_sets,
        as_routes,
    } = lexed;
    let dump = Dump {
        aut_nums: parse_lexed_aut_nums(aut_nums),
        as_sets: parse_lexed_as_sets(as_sets),
        route_sets: parse_lexed_route_sets(route_sets),
        peering_sets: parse_lexed_peering_sets(peering_sets),
        filter_sets: parse_lexed_filter_sets(filter_sets),
        as_routes: parse_lexed_as_routes(as_routes),
    };
    dump.log_count();
    dump
}

pub fn parse_lexed_aut_nums(lexed: Vec<lex::AutNum>) -> BTreeMap<u64, AutNum> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_aut_num(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_aut_num(aut_num: lex::AutNum) -> Result<(u64, AutNum)> {
    let num = parse_aut_num_name(&aut_num.name).context(format!("parsing {aut_num:?}"))?;
    let lex::AutNum {
        name: _,
        body,
        imports,
        exports,
    } = aut_num;
    let imports = parse_imports(imports);
    let exports = parse_imports(exports);
    Ok((
        num,
        AutNum {
            body,
            imports,
            exports,
        },
    ))
}

pub fn parse_aut_num_name(name: &str) -> Result<u64> {
    match regex_captures!(r"^AS(\d+)$"i, name) {
        Some((_, num)) => num
            .parse()
            .map_err(|err| Error::new(err).context(format!("parsing {name}"))),
        None => bail!("AS number name `{name}` does not match pattern"),
    }
}

pub fn parse_lexed_as_sets(lexed: Vec<lex::AsOrRouteSet>) -> BTreeMap<String, AsSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_as_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_as_set(lexed: lex::AsOrRouteSet) -> Result<(String, AsSet)> {
    if !is_as_set(&lexed.name) {
        bail!("invalid AS Set name in {lexed:?}");
    }
    let max_length = lexed.members.len();
    let mut members = Vec::with_capacity(max_length);
    let mut set_members = Vec::with_capacity(max_length);
    for member in lexed.members {
        let member = parse_as_name(member)
            .with_context(|| format!("parsing AS Set {}\n{}", lexed.name, lexed.body))?;
        match member {
            AsName::Any => bail!("AS Set {} contains `ANY`", lexed.name),
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

pub fn parse_lexed_route_sets(lexed: Vec<lex::AsOrRouteSet>) -> BTreeMap<String, RouteSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_route_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_route_set(lexed: lex::AsOrRouteSet) -> Result<(String, RouteSet)> {
    if !is_route_set_name(&lexed.name) {
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

pub fn parse_lexed_peering_sets(lexed: Vec<lex::PeeringSet>) -> BTreeMap<String, PeeringSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_peering_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
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

pub fn parse_lexed_filter_sets(lexed: Vec<lex::FilterSet>) -> BTreeMap<String, FilterSet> {
    lexed
        .into_par_iter()
        .filter_map(|l| parse_lexed_filter_set(l).map_err(|e| error!("{e:#}")).ok())
        .collect()
}

pub fn parse_lexed_filter_set(lexed: lex::FilterSet) -> Result<(String, FilterSet)> {
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
            .map(|f| parse_filter(f, &[]))
            .collect(),
    };
    Ok((lexed.name, filter_set))
}

pub fn parse_lexed_as_routes(
    as_routes: BTreeMap<String, Vec<String>>,
) -> BTreeMap<u64, Vec<IpNet>> {
    as_routes
        .into_iter()
        .filter_map(|as_route| {
            parse_lexed_as_route(&as_route)
                .map_err(|e| error!("Parsing routes for {as_route:?}: {e}."))
                .ok()
        })
        .collect()
}

pub fn parse_lexed_as_route((name, routes): &(String, Vec<String>)) -> Result<(u64, Vec<IpNet>)> {
    let num = parse_aut_num_name(name)?;
    let routes: Result<_> = routes.iter().map(|r| Ok(r.parse()?)).collect();
    let mut routes: Vec<_> = routes?;
    routes.sort_unstable();
    Ok((num, routes))
}
