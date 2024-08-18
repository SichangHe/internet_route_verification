//! The RPSL `as-set` and `as-num` directed graph.
use hashbrown::HashMap;
use petgraph::{
    algo::{dijkstra, is_cyclic_directed},
    dot::Dot,
    prelude::{DiGraph, NodeIndex},
};

#[cfg(test)]
mod tests;

/// AS Sets and AS Num membership graph.
/// The display format can be used with Graphviz to visualize the graph.
/// (Try print one out and paste it into Graphviz online.)
#[derive(Clone, Debug, Default)]
pub struct ASSetGraph {
    pub as_num_and_sets: HashMap<ASNumOrSet, NodeIndex>,
    /// Membership graph of AS Nums and AS sets.
    /// Edge weights are 1 except for pseudo sets.
    pub graph: DiGraph<ASNumOrSet, u32>,
}

impl std::fmt::Display for ASSetGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_dot())
    }
}

impl ASSetGraph {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            as_num_and_sets: HashMap::with_capacity(capacity),
            graph: DiGraph::with_capacity(capacity, capacity << 1),
        }
    }

    pub fn get_or_insert(&mut self, as_num_or_set: ASNumOrSet) -> NodeIndex {
        if let Some(index) = self.as_num_and_sets.get(&as_num_or_set) {
            *index
        } else {
            let index = self.graph.add_node(as_num_or_set.clone());
            _ = self.as_num_and_sets.insert(as_num_or_set, index);
            index
        }
    }

    /// Add each member in `members` to `set`.
    pub fn add_members<I>(&mut self, members: I, set: ASNumOrSet) -> (Vec<NodeIndex>, NodeIndex)
    where
        I: IntoIterator<Item = ASNumOrSet>,
    {
        let set_index = self.get_or_insert(set);
        let member_indexes = members
            .into_iter()
            .map(|member| {
                let edge_weight = if member.is_pseudo_set() { 0 } else { 1 };
                let member_index = self.get_or_insert(member);
                self.graph.add_edge(set_index, member_index, edge_weight);
                member_index
            })
            .collect();
        (member_indexes, set_index)
    }

    pub fn count_stats(&self, set_index: NodeIndex) -> ASSetGraphStats {
        let mut stats = ASSetGraphStats::default();

        let shortest_distances = dijkstra(&self.graph, set_index, None, |e| *e.weight());
        for (as_num_or_set, node_index) in &self.as_num_and_sets {
            if *node_index == set_index {
                continue;
            }

            match as_num_or_set {
                ASNumOrSet::Num(_) => {
                    stats.n_nums += 1;
                    if let Some(distance) = shortest_distances.get(node_index) {
                        stats.depth = stats.depth.max(*distance);
                    }
                }
                ASNumOrSet::Set(_) => {
                    if !as_num_or_set.is_pseudo_set() {
                        stats.n_sets += 1
                    }
                }
            }
        }

        stats
    }

    pub fn has_cycle(&self) -> bool {
        is_cyclic_directed(&self.graph)
    }

    pub fn to_dot(&self) -> Dot<&DiGraph<ASNumOrSet, u32>> {
        Dot::new(&self.graph)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ASSetGraphStats {
    pub n_sets: u32,
    pub n_nums: u32,
    pub depth: u32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ASNumOrSet {
    Num(u32),
    Set(String),
}

impl ASNumOrSet {
    pub fn set(s: &str) -> Self {
        Self::Set(s.into())
    }

    pub fn is_pseudo_set(&self) -> bool {
        match self {
            ASNumOrSet::Set(set) => set.contains('#'),
            _ => false,
        }
    }
}

impl std::fmt::Display for ASNumOrSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASNumOrSet::Num(num) => write!(f, "AS{num}"),
            ASNumOrSet::Set(set) => f.write_str(set),
        }
    }
}
