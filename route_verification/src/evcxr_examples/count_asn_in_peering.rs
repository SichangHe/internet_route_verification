use super::*;

/// Generate statistics for AS neighbors vs rules.
/// Copy this after running code from [`parse_bgp_lines`].
fn asn_in_peerings(query: QueryIr) {
    let mut total_import = 0usize;
    let mut total_export = 0usize;
    let mut single_any_import = 0usize;
    let mut single_any_export = 0usize;
    let mut single_asn_import = 0usize;
    let mut single_asn_export = 0usize;
    let mut complex_import = 0usize;
    let mut complex_export = 0usize;

    for an in query.aut_nums.values() {
        for (exchange, peerings, single_any, single_asn, complex) in [
            (
                &an.imports,
                &mut total_import,
                &mut single_any_import,
                &mut single_asn_import,
                &mut complex_import,
            ),
            (
                &an.exports,
                &mut total_export,
                &mut single_any_export,
                &mut single_asn_export,
                &mut complex_export,
            ),
        ] {
            for entry in exchange.entries_iter() {
                *peerings += entry.mp_peerings.len();
                for peering_action in &entry.mp_peerings {
                    if let AsExpr::Single(single) = &peering_action.mp_peering.remote_as {
                        match single {
                            AsName::Any => *single_any += 1,
                            AsName::Num(_) => *single_asn += 1,
                            _ => (),
                        }
                    } else {
                        *complex += 1;
                    }
                }
            }
        }
    }

    println!("{total_import} total imports, {single_any_import} single Any imports, {single_asn_import} single ASN imports, {complex_import} complex imports.");
    println!("{total_export} total exports, {single_any_export} single Any exports, {single_asn_export} single ASN exports, {complex_export} complex exports.");
}
