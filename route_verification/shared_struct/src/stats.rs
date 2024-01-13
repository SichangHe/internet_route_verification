#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Counts {
    pub lex_skip: usize,
    pub syntax_err: usize,
    pub unknown_lex_err: usize,
    /// Unknown path attributes (the basic component of <filter>) when parsing.
    pub parse_path_attr: usize,
    /// Invalid name parsing AutNum.
    pub parse_aut_num: usize,
    /// Error parsing AS Set.
    pub parse_as_set: usize,
    /// Invalid name when parsing Route Set.
    pub parse_route_set: usize,
    /// Invalid name when parsing Peering Set.
    pub parse_peering_set: usize,
    /// Invalid name when parsing Filter Set.
    pub parse_filter_set: usize,
    /// Invalid routes when parsing AS Routes.
    pub parse_as_route: usize,
    /// PeerAS points to invalid AS name.
    pub peer_as_point: usize,
    /// Using PeerAS but `mp-peering` is not a single AS expression.
    pub complex_peer_as: usize,
}

impl std::ops::Add for Counts {
    type Output = Counts;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lex_skip: self.lex_skip + rhs.lex_skip,
            syntax_err: self.syntax_err + rhs.syntax_err,
            unknown_lex_err: self.unknown_lex_err + rhs.unknown_lex_err,
            parse_path_attr: self.parse_path_attr + rhs.parse_path_attr,
            parse_aut_num: self.parse_aut_num + rhs.parse_aut_num,
            parse_as_set: self.parse_as_set + rhs.parse_as_set,
            parse_route_set: self.parse_route_set + rhs.parse_route_set,
            parse_peering_set: self.parse_peering_set + rhs.parse_peering_set,
            parse_filter_set: self.parse_filter_set + rhs.parse_filter_set,
            parse_as_route: self.parse_as_route + rhs.parse_as_route,
            peer_as_point: self.peer_as_point + rhs.peer_as_point,
            complex_peer_as: self.complex_peer_as + rhs.complex_peer_as,
        }
    }
}

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            lex_skip,
            syntax_err,
            unknown_lex_err,
            parse_path_attr,
            parse_aut_num,
            parse_as_set,
            parse_route_set,
            parse_peering_set,
            parse_filter_set,
            parse_as_route,
            peer_as_point,
            complex_peer_as,
        } = self;
        [
            (lex_skip, "skips during lexing"),
            (syntax_err, "syntax errors"),
            (unknown_lex_err, "unknown lex errors"),
            (parse_path_attr, "unknown path attributes in filter"),
            (parse_aut_num, "invalid AutNum names"),
            (parse_as_set, "invalid names parsing AS Sets"),
            (parse_route_set, "invalid Route Set names"),
            (parse_peering_set, "invalid Peering Set names"),
            (parse_filter_set, "invalid Filter Set names"),
            (parse_as_route, "invalid AS Route"),
            (peer_as_point, "PeerAS points to invalid AS name"),
            (complex_peer_as, "complex PeerAS"),
        ]
        .into_iter()
        .filter(|(field, _)| **field > 0)
        .enumerate()
        .try_for_each(|(index, (field, desc))| {
            if index > 0 {
                f.write_fmt(format_args!(", {field} {desc}"))
            } else {
                f.write_fmt(format_args!("{field} {desc}"))
            }
        })
    }
}
