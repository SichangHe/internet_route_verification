use super::*;

/// Distribution of different types of <filter>s.
/// Copy the content after running code from [`parse_bgp_lines`].
fn filter_persentages(query: QueryIr) {
    #[derive(Copy, Clone, Debug, Default)]
    struct Count {
        filter_set: usize,
        any: usize,
        addr_prefix_set: usize,
        route_set: usize,
        as_num: usize,
        as_set: usize,
        as_path_re: usize,
        peer_as: usize,
        and: usize,
        or: usize,
        not: usize,
        group: usize,
        community: usize,
        unknown: usize,
    }
    impl Count {
        fn add_filter(&mut self, filter: &Filter) {
            match filter {
                Filter::FilterSet(_) => self.filter_set += 1,
                Filter::Any => self.any += 1,
                Filter::AddrPrefixSet(_) => self.addr_prefix_set += 1,
                Filter::RouteSet(_, _) => self.route_set += 1,
                Filter::AsNum(_, _) => self.as_num += 1,
                Filter::AsSet(_, _) => self.as_set += 1,
                Filter::AsPathRE(_) => self.as_path_re += 1,
                Filter::PeerAS => self.peer_as += 1,
                Filter::And { left, right } => self.and += 1,
                Filter::Or { left, right } => self.or += 1,
                Filter::Not(_) => self.not += 1,
                Filter::Group(_) => self.group += 1,
                Filter::Community(_) => self.community += 1,
                Filter::Unknown(_) => self.unknown += 1,
            }
        }
        fn report_percentages(self) -> String {
            let Self {
                filter_set,
                any,
                addr_prefix_set,
                route_set,
                as_num,
                as_set,
                as_path_re,
                peer_as,
                and,
                or,
                not,
                group,
                community,
                unknown,
            } = self;
            let total = (filter_set
                + any
                + addr_prefix_set
                + route_set
                + as_num
                + as_set
                + as_path_re
                + peer_as
                + and
                + or
                + not
                + group
                + community
                + unknown) as f64;
            let filter_set_percentage = filter_set as f64 * 100.0 / total;
            let any_percentage = any as f64 * 100.0 / total;
            let addr_prefix_set_percentage = addr_prefix_set as f64 * 100.0 / total;
            let route_set_percentage = route_set as f64 * 100.0 / total;
            let as_num_percentage = as_num as f64 * 100.0 / total;
            let as_set_percentage = as_set as f64 * 100.0 / total;
            let as_path_re_percentage = as_path_re as f64 * 100.0 / total;
            let peer_as_percentage = peer_as as f64 * 100.0 / total;
            let and_percentage = and as f64 * 100.0 / total;
            let or_percentage = or as f64 * 100.0 / total;
            let not_percentage = not as f64 * 100.0 / total;
            let group_percentage = group as f64 * 100.0 / total;
            let community_percentage = community as f64 * 100.0 / total;
            let unknown_percentage = unknown as f64 * 100.0 / total;
            format!(
                "filter_set: {:.2}%, any: {:.2}%, addr_prefix_set: {:.2}%, route_set: {:.2}%, as_num: {:.2}%, as_set: {:.2}%, as_path_re: {:.2}%, peer_as: {:.2}%, and: {:.2}%, or: {:.2}%, not: {:.2}%, group: {:.2}%, community: {:.2}%, unknown: {:.2}%",
                filter_set_percentage,
                any_percentage,
                addr_prefix_set_percentage,
                route_set_percentage,
                as_num_percentage,
                as_set_percentage,
                as_path_re_percentage,
                peer_as_percentage,
                and_percentage,
                or_percentage,
                not_percentage,
                group_percentage,
                community_percentage,
                unknown_percentage,
            )
        }
    }

    let mut count = Count::default();
    query
        .aut_nums
        .values()
        .flat_map(|an| [&an.imports, &an.exports])
        .flat_map(|port| port.entries_iter())
        .for_each(|entry| count.add_filter(&entry.mp_filter));
    println!("{:#?}", count);
    println!("{}", count.report_percentages());
}
