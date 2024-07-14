# Artifacts

Follow the instructions below to reproduce the artifacts.

## Setup

1. Get yourself a UNIX environment. Try WSL if you are on Windows.

1. Make sure you have these CLI tools:

    ```text
    git rg bgpdump
    ```

1. Make sure you
    [have a Rust toolchain installed](https://www.rust-lang.org/tools/install).

1. Make sure you have the Evcxr REPL installed. If not, install it with:

    ```sh
    cargo install evcxr_repl
    ```

1. Make sure you [have Rye installed](https://rye.astral.sh/).

1. Clone and enter this repository:

    ```sh
    git clone --depth=1 https://github.com/SichangHe/internet_route_verification.git
    cd internet_route_verification
    ```

    Henceforth, we will call this directory `./`.

## Data preparation

1. Download the source data from [Raw data used, for
    reproducibility](https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data)
    and follow the instruction there to unpack them to
    the correct directory structure.

    > The source data contain two parts.
    > The RIBs (Routing Information Bases)
    > are originally downloaded using `./download_ribs.py`.
    > The IRR (Internet Route Registry)
    > dumps are downloaded from the FTP servers listed in `README.md`;
    > they are irreproducible because they are from the pass and
    > IRRs do not keep archives.

1. To reproduce the IR from the RPSL data,
    `cd` to `./route_verification/` and set up the Rye environment:

    ```sh
    rye sync
    ```

    If you do not have the
    [Rye global shim](https://rye.astral.sh/guide/shims/) enabled,
    you need to activate the virtual environment created by Rye.

    ```sh
    . .venv/bin/activate # Or `activate.zsh`, etc., corresponding to your shell.
    ```

    Then, at `./route_verification/`,
    run the command below to produce the IR at `./parsed_all/` and
    the log at `parse_out.txt`:

    ```sh
    cargo r --release -- parse_ordered ../data/irrs/priority/apnic.db.* ../data/irrs/priority/afrinic.db ../data/irrs/priority/arin.db ../data/irrs/priority/lacnic.db ../data/irrs/priority/ripe.db ../data/irrs/backup/radb.db ../data/irrs/backup/altdb.db ../data/irrs/backup/idnic.db ../data/irrs/backup/jpirr.db ../data/irrs/backup/level3.db ../data/irrs/backup/nttcom.db ../data/irrs/backup/reach.db ../data/irrs/backup/tc.db ../parsed_all/ | tee parse_out.txt
    ```

1. To reproduce the BGP dump analysis results obtained from the Evcxr shell,
    `cd` to `./` and start Evcxr:

    ```sh
    evcxr
    ```

    Inside the shell,
    paste in two blocks of code from
    `./route_verification/src/evcxr_examples.rs`.
    The first block goes from `:opt 3` to the end of the block of `use`;
    it imports the dependencies.
    The second block is the content of `parse_bgp_lines`,
    excluding the `Ok(())`; it loads the data.
    This takes a while and does not need supervision,
    so just leave it there and do something else.

    <details>
    <summary>The code to copy, repeated here.</summary>

    ```rust
    :opt 3
    :dep anyhow
    :dep dashmap
    :dep hashbrown
    :dep route_verification = { path = "route_verification" }
    :dep rayon
    :dep itertools
    :dep serde_json
    // */
    use anyhow::Result;
    use dashmap::{DashMap, DashSet};
    use hashbrown::{HashMap, HashSet};
    use itertools::multiunzip;
    use rayon::prelude::*;
    use route_verification::as_rel::*;
    use route_verification::bgp::stats::*;
    use route_verification::bgp::*;
    use route_verification::fs::open_file_w_correct_encoding;
    use route_verification::ir::*;
    use route_verification::irr::split_commas;
    use route_verification::lex::{
        expressions, io_wrapper_lines, lines_continued, rpsl_objects, RpslExpr,
    };
    use std::{
        env,
        fs::{read_dir, read_to_string, File},
        io::{prelude::*, BufReader, BufWriter},
        ops::Add,
        time::Instant,
    };

    let db = AsRelDb::load_bz("data/20230701.as-rel.bz2")?;
    let parsed = Ir::pal_read("parsed_all")?;
    println!(
        "{}",
        serde_json::to_string(parsed.aut_nums.get(&33549).unwrap())?
    );
    let query: QueryIr = QueryIr::from_ir_and_as_relationship(parsed, &db);
    println!("{:#?}", query.aut_nums.iter().next());
    let mut bgp_lines: Vec<Line> = parse_mrt("data/mrts/rib.20230619.2200.bz2")?;
    ```

    We do not need Polars.

    </details>

    > Tip:
    > When copying contents of functions in
    > `./route_verification/src/evcxr_examples/`, always omit `Ok(())`.

1. To reproduce the intermediate data the scripts in
    `./scripts/scripts/` download automatically from GitHub,
    `cd` to `scripts/` and… <!-- TODO: Then what? -->

## Results to reproduce

- [ ] intro: 53.2% of ASes not declaring any policies.
    <https://github.com/SichangHe/internet_route_verification/issues/161>
- [ ] intro:
    a large portion of interconnections present in BGP routes (40.4%)
    cannot be verified using the RPSL due to missing information.
    <https://github.com/SichangHe/internet_route_verification/issues/162>
- [ ] intro: For interconnections covered in the RPSL,
    we observe a high fraction (29.3%) of strict matches.
    We explain most mismatches (19.0%)
    by
    six common mistakes we identified
    <https://github.com/SichangHe/internet_route_verification/issues/162>
- [ ] sec 3: RPSLyzer parses the 13 IRRs listed in Table 1,
    totaling 7.1 GiB of data, and exports the IR,
    all in under five minutes on an Apple M1
- [ ] sec 4: Table 1.
    <https://github.com/SichangHe/internet_route_verification/issues/126>.

    ```sh
    ls -l | awk 'BEGIN { printf "%-50s %10s MiB\n", "File", "Size" } NR>1 { size=$5/1024/1024; printf "%-50s %10.3f MiB\n", $9, size }'
    ```

    35.4% of aut-nums contain no rules,
    <https://github.com/SichangHe/internet_route_verification/issues/60>,
    10.9% define at least 10 rules, and 0.13% (101 aut-nums)
    define over 1000 rules.
    <https://github.com/SichangHe/internet_route_verification/issues/122>
- [ ] sec 4:
    no significant correlation between how many rules an AS defines and
    how many neighbors, customers, peers,
    or providers it has in CAIDA’s AS-relationship database.
    <https://github.com/SichangHe/internet_route_verification/issues/19>.
    <https://github.com/SichangHe/internet_route_verification/issues/95>.
    <https://github.com/SichangHe/internet_route_verification/issues/109>
- [ ] sec 4: Almost all (98.1%)
    peering definitions comprise a single ASN or ANY.
    <https://github.com/SichangHe/internet_route_verification/issues/107>.
    <https://github.com/SichangHe/internet_route_verification/issues/64>
- [ ] sec 4: Most (95.0%)
    ASes with rules only specify simple filters compatible with BGPq4.
    <https://github.com/SichangHe/internet_route_verification/issues/64>
- [ ] sec 4:
    Table 2 shows that 60.4% of aut-num and 31.7% of
    as-set objects are referenced in filter definitions.
    <https://github.com/SichangHe/internet_route_verification/issues/123>
- [ ] sec 4: most filters are either an as-set (43.4%) or ASN (24.1%).
    <https://github.com/SichangHe/internet_route_verification/issues/159>
- [ ] sec 4: Our IRR dumps contain 3,904,352 route objects,
    corresponding to 3,367,914 unique prefix-origin pairs and
    2,817,344 unique prefixes.
    697,269 (24.7%) have multiple route objects defined,
    among which 404,901 (58.1%)
    prefixes have route objects with different origins.
    Furthermore, 469,003 (67.3%)
    prefixes have route objects defined by multiple operators.
    <https://github.com/SichangHe/internet_route_verification/issues/138>
- [ ] sec 4: Among 53,268 as-set objects across all IRRs, 7754 (14.6%)
    have no members. 17,434 (32.7%) as-sets contain only one member AS.
    A few (772, 1.4%) extremely large as-sets have more than 10,000 members.
    <https://github.com/SichangHe/internet_route_verification/issues/114>.
    We find that 13,602 (25.5%) of as-sets recursively contain other as-sets,
    among which 3050 (22.4%) form loops and 3129 (23.0%) have depth 5 or more.
    <https://github.com/SichangHe/internet_route_verification/issues/114#issuecomment-1903153622>
- [ ] sec 4: RPSLyzer found 663 syntax errors, 12 invalid as-set names,
    and 17 invalid route-set names.
    <https://github.com/SichangHe/internet_route_verification/issues/57>
- [ ] sec 4: Common syntax errors include out-of-place text,
    such as broken comma-separated lists, misplaced comments,
    invalid RPSL keywords in import and export rules, or plain typos.
    <https://github.com/SichangHe/internet_route_verification/discussions/39>
- [ ] sec 5:
    We ignore 0.06% of routes that are direct exports from
    the collector’s peer ASes and 0.03% of
    routes whose AS-paths contain BGP AS-sets.
    <https://github.com/SichangHe/internet_route_verification/issues/111>
- [ ] sec 5:
    Verifying the 779.3 million routes in all 60 BGP dumps took 2 h 49 m and
    less than 2 GiB of RAM.
    <https://github.com/SichangHe/internet_route_verification/issues/157>
- [ ] sec 5.1: More than half (6664, 64.4%)
    of transit ASes specify themselves as an export rule’s filter.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.1: 3090 (29.8%)
    transit ASes specify a customer AS C in both an import rule’s peering and
    filter.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.1: A few (46, 0.44%)
    transit ASes only specify rules for their providers.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.1: A few (46, 0.44%)
    transit ASes only specify rules for their providers.
    <https://github.com/SichangHe/internet_route_verification/issues/134>
- [ ] sec 5.2: Figure 2
- [ ] sec 5.2: The majority (61,725, 74.4%)
    of ASes have all imports and exports with identical statuses.
    We identified 14.2% of ASes with 100% of propagation verified (yellow),
    51.6% lacking RPSL information (“unrecorded”, green),
    0.34% that only use relaxed filters (blue),
    and 6.9% with only safelisted relationships (red).
    <https://github.com/SichangHe/internet_route_verification/issues/90>
- [ ] sec 5.2: ASes with skipped verifications only constitute 0.03% of ASes.
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2085134606>
- [ ] sec 5.2: Out of the 54.9% of ASes with unrecorded cases,
    most can be explained by 27.2% of ASes missing aut-num objects and 24.2% of
    aut-nums with no rules.
    Excluding ASes with skipped or unrecorded cases,
    we find more ASes with verified (76.3%) or special-cased (62.5%)
    routes than ASes with unverified routes (23.1%).
    <https://github.com/SichangHe/internet_route_verification/issues/154>
- [ ] sec 5.2:
    25,596 ASes with at least one special-cased import or export
    (30.9% out of all ASes).
    Among these ASes, more incorrectly allow customer route exports (994,
    “export self”) than imports (325, “import customer”).
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2094205769>
- [ ] sec 5.2:
    most of the special cases are due to uphill propagation with
    no matching rules (23,298 ASes) or missing route objects (5181 ASes).
    <https://github.com/SichangHe/internet_route_verification/issues/78>
- [ ] sec 5.2: Figure 3.
- [ ] sec 5.2: For imports,
    we find 96% of AS pairs have a single consistent status;
    this number is 92% for exports.
    <https://github.com/SichangHe/internet_route_verification/issues/96>
- [ ] sec 5.2: over half of AS pairs have unverified routes (418,328, 63.0%).
    <https://github.com/SichangHe/internet_route_verification/issues/94#issuecomment-1822005650>.
    most of them (98.98%)
    fail verification because the relationship is not declared in the RPSL.
    <https://github.com/SichangHe/internet_route_verification/issues/117>
- [ ] sec 5.2: Figure 4
- [ ] sec 5.2: Only 6.6% of routes have the same status across all hops,
    captured by having a bar of single color (1.6% verified, 3.0% unrecorded,
    and 1.6% unverified).
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2085328442>.
    <https://github.com/SichangHe/internet_route_verification/issues/88>.
    <https://github.com/SichangHe/internet_route_verification/issues/38#issuecomment-1725626125>
- [ ] sec 5.2:
    We also assess the verification status of the first hop in ASpaths…
    Unfortunately, the results are similar (not shown).
    <https://github.com/SichangHe/internet_route_verification/issues/141>
- [ ] Appendix B Nonstandard features:
    two cases of non-standard but common syntax used by operators (4724 times…)
- [ ] Appendix B Limitations:

    > We leave the handling of
    > 60 rules whose filters contain AS-path regex with ASN ranges (21 rules)

    Run at `./data/irrs/`
    ([#106](https://github.com/SichangHe/internet_route_verification/issues/106)):

    ```sh
    rg --no-ignore -c '<.*\[\s*AS\d+\s*-\s*AS\d+\s*\].*>'
    ```

    <!-- FIXME: This says 19 instead of 21. The text is outdated. -->

    > or samepattern unary postfix operators (e.g., ~*, 39 rules)
    > as future work.

    Run at `./data/irrs/`
    ([#113](https://github.com/SichangHe/internet_route_verification/issues/113)):

    ```sh
    rg --no-ignore -c '<.*~.*>'
    ```

    > we ignore 54 rules with BGP community attributes in their filters.

    In the Evcxr shell,
    follow the instruction in
    `./route_verification/src/evcxr_examples/community_filter.rs`,
    then evaluate the variable `count`
    ([#158](https://github.com/SichangHe/internet_route_verification/issues/158)).

- [ ] appendix C:
    <https://github.com/SichangHe/internet_route_verification/issues/83>
- [ ] appendix D: Figure 5.
- [ ] appendix D:
    the most common unrecorded case is 22,562 ASes not having an aut-num
    object.
    The second most common type is for 20,048 ASes that have zero import
    (or export) rules when verifying an import (or export).
- [ ] appendix D:
    Fewer ASes have rules that refer to ASes with
    no originating route objects (zero-route ASes, 2706),
    or set objects (as-set, route-set, peering-set, and filter-set)
    missing in the IRRs (414).
- [ ] appendix D: Figure 6
- [ ] appendix D: A small portion (325, 0.4%) of ASes use “import customer”,
    while more (994, 1.2%) use “export self”.
    <https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2094205769>.
    A significant portion (6.2%) of ASes have missing route objects.
    ASes that have uphill propagation with
    no matching RPSL rules occupy a large 28.1% of all ASes,
    much more than the 12.4% of ASes with unverified routes.
    <https://github.com/SichangHe/internet_route_verification/issues/78>
