# Artifacts

Follow the instructions below to reproduce the artifacts.
You may also want to check out `ALIASES.md` to help understand the names in
the code output.

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

### Downloading the source data

Download the source data from [this release](https://github.com/SichangHe/internet_route_verification/releases/tag/raw-data) on GitHub.
This data ensures we can reproduce the results in the paper.
Run the script in **TODO** to download and unpack the data into the `./data` directory.

> [!NOTE]\
> The source data contain three parts.
> The RIBs (Routing Information Bases)
> are originally downloaded using `./download_ribs.py`.
> The IRR (Internet Route Registry)
> dumps are downloaded from the FTP servers listed in `./README.md`;
> they cannot be easily reobtained because
> IRRs do not keep archives.
> The AS-relationship Database is originally downloaded from
> [CAIDA's AS-relationship
> dataset](https://data.caida.org/datasets/2013-asrank-data-supplement/).

<!-- TODO: Comply with CAIDA's AUA. -->

> [!TIP]\
> If your editor is sufficiently Vim-like,
> you can open directories in this document by pressing <kbd>gf</kbd> on
> their paths.

### Generating the Intermediate Representation (IR)

To reproduce the IR from the RPSL data,
`cd` to `./route_verification/` and set up the Rye environment:

```sh
rye sync
```

If you do not have the [Rye global shim](https://rye.astral.sh/guide/shims/)
enabled, you need to activate the virtual environment created by Rye.

```sh
. .venv/bin/activate # Or `activate.zsh`, etc., corresponding to your shell.
```

Then, at `./route_verification/`,
run the command below to produce the IR at `./parsed_all/` and
the log at `./route_verification/parse_out.txt`:

```sh
time cargo r --release -- parse_ordered \
    ../data/irrs/priority/apnic.db.* \
    ../data/irrs/priority/afrinic.db \
    ../data/irrs/priority/arin.db \
    ../data/irrs/priority/lacnic.db \
    ../data/irrs/priority/ripe.db \
    ../data/irrs/backup/idnic.db \
    ../data/irrs/backup/jpirr.db \
    ../data/irrs/backup/radb.db \
    ../data/irrs/backup/nttcom.db \
    ../data/irrs/backup/level3.db \
    ../data/irrs/backup/tc.db \
    ../data/irrs/backup/reach.db \
    ../data/irrs/backup/altdb.db \
    ../parsed_all/ |& tee parse_out.txt
```

<!-- FIXME: Current data have slightly different ordering. -->

The time taken, Time-IR, is the time to parse the IRR data into the IR.
The parsing order, IRR-Order, is the order in Table 1. The log is Parse-Log.

> [!NOTE]\
> We use these hyphenated capitalized names to refer to values,
> so that you can easily search for them when you encounter them below.

### Shell-Evcxr setup

To reproduce the BGP dump analysis results obtained from the Evcxr shell,
launch a separate shell at `./` and start Evcxr:

```sh
evcxr
```

Keep this shell open. We will call it Shell-Evcxr and run Rust scripts in it.

Inside the Shell-Evcxr,
paste in two blocks of code from `./route_verification/src/evcxr_examples.rs`.
The first block goes from `:opt 3` to the end of the block of `use`;
it imports the dependencies.
The second block is the content of
`parse_bgp_lines` excluding the `parse_mrts` line; it loads the data.
This takes a while and does not need monitoring, so,
just leave it there and do something else.

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

let db = AsRelDb::load_bz("data/20230701.as-rel.bz2").unwrap();
let ir = Ir::pal_read("parsed_all").unwrap();
println!(
    "{}",
    serde_json::to_string(ir.aut_nums.get(&33549).unwrap()).unwrap()
);
let query: QueryIr = QueryIr::from_ir_and_as_relationship(ir.clone(), &db);
println!("{:#?}", query.aut_nums.iter().next());
```

We do not need Polars.

</details>

> [!TIP]\
> If your editor is sufficiently Vim-like,
> you can copy the content of a function with <kbd>yi{</kbd>.

> [!NOTE]\
> Evcxr compiles all input Rust code, a slow process,
> so it takes a while before Evcxr has any reaction.

### Shell-IPython setup

To reproduce some of the result analysis done in the IPython shell,
launch a separate shell at `./scripts/` and set up the Rye environment:

```sh
rye sync
```

Activate the virtual environment:

```sh
. .venv/bin/activate # Or `activate.zsh`, etc., corresponding to your shell.
```

Then, launch IPython:

```sh
ipython
```

Keep this shell open.
We will call it Shell-IPython and run Python scripts in it.

> [!Tip]\
> IPython garbage collection is wonky,
> so you may want to restart it when your memory usage blows up.

## Generating intermediate results in CSV

`./scripts/scripts/csv_files.py` downloads these intermediate results
automatically from GitHub, so you do not need to generate them to proceed.
However, if you would like to generate some of these CSV files,
please follow the instructions below.

After generating files using Shell-Evcxr, the files are written to `./`,
so you may need to Gzip and move the generated files to `./scripts/`.

> [!NOTE]\
> After some of the instructions,
> we attach the corresponding GitHub issue number for reference.

### Generating `as_neighbors_vs_rules`

In Shell-Evcxr,
follow the instructions in
`./route_verification/src/evcxr_examples/as_neighbors_vs_rules.rs`.
[#60](https://github.com/SichangHe/internet_route_verification/issues/60).

### Generating `as_compatible_with_bgpq3`

In Shell-Evcxr,
follow the instructions in
`./route_verification/src/evcxr_examples/as_compatible_w_bgpq3.rs`.
[#64](https://github.com/SichangHe/internet_route_verification/issues/64).

### Generating `*_appearances_in_rules`

In Shell-Evcxr,
follow the instructions in
`./route_verification/src/evcxr_examples/object_referred_in_rules.rs`.
This generates `as_num_appearances_in_rules`, `as_set_appearances_in_rules`,
`filter_set_appearances_in_rules`, `peering_set_appearances_in_rules`,
`route_set_appearances_in_rules`.
[#123](https://github.com/SichangHe/internet_route_verification/issues/123).

### Generating `route_objects_defined_multiple_times`

At `./route_verification/stat_route_objects/`, run:

```sh
cargo r --release -- ../../data/irrs/backup ../../data/irrs/priority/
```

This script is sequential and thus slow,
so just leave it there and do something else.
It does not generate a CSV file but a JSON file.
The results are written to `./route_verification/stat_route_objects/` and
could be moved to `./scripts/` manually.
[#138](https://github.com/SichangHe/internet_route_verification/issues/138).

### Generating `as_set_graph_stats`

In Shell-Evcxr,
follow the instructions in
`./route_verification/src/evcxr_examples/as_set_graphing.rs`.

### Generating `*_stats_all`

At `./route_verification/rib_stats`, run:

```sh
time cargo r --release
```

This generates `as_stats_all`, `as_pair_stats_all`, `route_stats_all`,
and `route_first_hop_stats_all` at `./route_verification/rib_stats/`.
[#157](https://github.com/SichangHe/internet_route_verification/issues/157).
<!-- FIXME: This script generates `all4/`, but we are still using `all3/` CSV files. -->

This script is very computationally expensive, so you only want to run it once.
We call the time taken RIB-Stats-Time.

<details>
<summary>
You may want to monitor the RAM usage of this, RIB-Stats-RAM,
to reproduce some results below.
</summary>

After launching the script,
go to `top` or a similar tool to find its PID and assign that to
`WATCHED_PID` in another shell. Then, run:

```sh
while ps -p $WATCHED_PID --no-headers --format "rss" >> ram.txt; do
    sleep 30
done
```

Or, in fish:

```fish
while ps -p $WATCHED_PID --no-headers --format "rss" >> ram.txt
    sleep 30
end
```

This will write the RAM usage to `ram.txt` every 30 seconds.
You can change the interval,
but keep in mind that the script may go on for hours and blow up your log file.

To find the largest number in `ram.txt`:

```sh
awk 'BEGIN { max = 0 } { if ($1 > max) max = $1 } END { print max }' ram.txt
```

</details>

### Generating `route_all_*_stats` and `route_first_hop_all_*_stats`

After generating `*_stats_all` (above), at `./scripts/`, run:

```sh
cargo r --release
```

## Results to reproduce

> [!TIP]\
> You can use the checkboxes in this file to track your progress.

- [ ] INTRODUCTION:

    > 53.2% of ASes not declaring any policies.

    Run the script in
    [#161](https://github.com/SichangHe/internet_route_verification/issues/161)
    in Shell-IPython.

    <details><summary>The code to copy, repeated here.</summary>

    ```python
    from scripts.csv_files import *
    import pandas as pd
    df = pd.read_csv(as_neighbors_vs_rules.path)
    n_all = len(df)
    df["rules"] = df["import"] + df["export"]
    df_w_rule = df[df["rules"] > 0]
    n_w_rule = len(df_w_rule)
    percentage = f"{n_w_rule * 100.0 / n_all:.1f}"
    percentage
    ```

    </details>

- [ ] INTRODUCTION:

    > a large portion of interconnections present in BGP routes (40.4%)
    > cannot be verified using the RPSL due to missing information.

    And:

    > For interconnections covered in the RPSL,
    > we observe a high fraction (29.3%) of strict matches.
    > We explain most mismatches (19.0%) by six common mistakes we identified

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.imports_exports import main
    main()
    ```

    </details>

    The corresponding results are `total unrec`, `total ok`, and `total meh`.

    [#162](https://github.com/SichangHe/internet_route_verification/issues/162).

- [ ] 3 PARSING THE RPSL:

    > RPSLyzer parses the 13 IRRs listed in Table 1, totaling 7.1 GiB of data,
    > and exports the IR, all in under five minutes on an Apple M1.

    This size is the size of `./data/irrs/`. The parsing time is Time-IR.
    <!-- FIXME: The size is 6.9 GiB after we deduplicated backups. -->

- [ ] 4 RPSL USE IN THE WILD:

    > Table 1: IRRs used, grouped and ordered by priority.

    The order is the IRR-Order. The total counts are in Parse-Log.
    The sizes are obtained by running the script at `./data/irrs/priority/` and
    `./data/irrs/backup/`:

    ```sh
    ls -l | awk 'BEGIN { printf "%-50s %10s MiB\n", "File", "Size" } NR>1 { size=$5/1024/1024; printf "%-50s %10.3f MiB\n", $9, size }'
    ```

    <!-- TODO: Find the script to count for each IRR.
    The commit to the text is:
    1a4ccc8e fully fill IRR table, 2023-11-20 16:02:24 +0800.
    I cannot find the script, though. -->

    [#126](https://github.com/SichangHe/internet_route_verification/issues/126).

- [ ] 4 RPSL USE IN THE WILD:

    > 35.4% of aut-nums contain no rules, 10.9% define at least 10 rules,
    > and 0.13% (101 aut-nums) define over 1000 rules.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.csv_files import *
    import pandas as pd
    df_raw = pd.read_csv(as_neighbors_vs_rules.path)
    df = df_raw.drop(df_raw[df_raw["import"] == -1].index)
    df["rules"] = df["import"] + df["export"]
    n_all = len(df)
    n_wo_rule = len(df[df["rules"] == 0])
    print(f"{n_wo_rule} aut-nums ({n_wo_rule * 100.0 / n_all:.1f}%) contain no rules.")
    n_over_1000 = len(df[df["rules"] >= 1000])
    print(f"{n_over_1000} aut-nums ({n_over_1000 * 100.0 / n_all:.2f}%) define over 1000 rules.")
    ```

    </details>

    [#137](https://github.com/SichangHe/internet_route_verification/issues/137).
    <!-- FIXME: It now says 35.2%. -->

- [ ] 4 RPSL USE IN THE WILD:

    > no significant correlation between how many rules an AS defines and
    > how many neighbors, customers, peers,
    > or providers it has in CAIDA’s AS-relationship database.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.as_all_corr import main
    main()
    ```

    </details>

    [#19](https://github.com/SichangHe/internet_route_verification/issues/19),
    [#95](https://github.com/SichangHe/internet_route_verification/issues/95),
    and especially
    [#109](https://github.com/SichangHe/internet_route_verification/issues/109).

- [ ] 4 RPSL USE IN THE WILD:

    > Almost all (98.1%) peering definitions comprise a single ASN or ANY.

    In Shell-Evcxr,
    follow the instructions in
    `./route_verification/src/evcxr_examples/count_asn_in_peering.rs`.
    <!-- FIXME: It now says 98.4%. -->

    [#107](https://github.com/SichangHe/internet_route_verification/issues/107).

- [ ] 4 RPSL USE IN THE WILD:

    > Most (95.0%)
    > ASes with rules only specify simple filters compatible with BGPq4.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.csv_files import *
    import pandas as pd
    df_raw = pd.read_csv(as_neighbors_vs_rules.path)
    df_raw["rules"] = df_raw["import"] + df_raw["export"]
    df = df_raw.drop(df_raw[df_raw["rules"] <= 0].index)
    n_have_rule = len(df)
    bgpq3_compatible = pd.read_csv(as_compatible_with_bgpq3.path)["as_compatible_w_bgpq3"]
    df_bgpq3_compatible = df[df["aut_num"].isin(bgpq3_compatible) & (df["rules"] > 0)]
    n_bgpq3_compatible = len(df_bgpq3_compatible)
    print(
        f"{n_bgpq3_compatible} ASes ({n_bgpq3_compatible * 100.0 / n_have_rule:.1f}%) are compatible with BGPq3 with the ASes that have rules."
    )
    ```

    </details>

    Note that ASes compatible with BGPq3 are also compatible with BGPq4.
    [#64](https://github.com/SichangHe/internet_route_verification/issues/64).
    <!-- FIXME: It now says 94.5%. -->

- [ ] 4 RPSL USE IN THE WILD:

    > Table 2 shows that 60.4% of aut-num and 31.7% of
    > as-set objects are referenced in filter definitions.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.object_appearance_table import main
    main()
    ```

    </details>

    The results are in the `\%\filter` row.
    [#123](https://github.com/SichangHe/internet_route_verification/issues/123).

- [ ] 4 RPSL USE IN THE WILD:

    > most filters are either an as-set (43.4%) or ASN (24.1%).

    The results are the percentages of `as_set` and `as_num`.
    [#159](https://github.com/SichangHe/internet_route_verification/issues/159).

- [ ] 4 RPSL USE IN THE WILD:

    > Our IRR dumps contain 3,904,352 route objects,
    > corresponding to 3,367,914 unique prefix-origin pairs and
    > 2,817,344 unique prefixes.
    > 697,269 (24.7%) have multiple route objects defined,
    > among which 404,901 (58.1%)
    > prefixes have route objects with different origins.
    > Furthermore, 469,003 (67.3%)
    > prefixes have route objects defined by multiple operators.

    Half of the information is from the printout of
    generating `route_objects_defined_multiple_times` (see above).

    <details>
    <summary>Run this script in Shell-IPython for the other half.</summary>

    ```python
    from scripts.stats.route_objects import main
    main()
    ```

    </details>

    [#138](https://github.com/SichangHe/internet_route_verification/issues/138)

- [ ] 4 RPSL USE IN THE WILD:

    > Among 53,268 as-set objects across all IRRs, 7754 (14.6%)
    > have no members. 17,434 (32.7%) as-sets contain only one member AS.
    > A few (772, 1.4%) extremely large as-sets have more than 10,000 members.

    And:

    > We find that 13,602 (25.5%) of as-sets recursively contain other as-sets,
    > among which 3050 (22.4%) form loops and 3129 (23.0%)
    > have depth 5 or more.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.as_set_size_fitting import main
    main()
    ```

    </details>

    [#114](https://github.com/SichangHe/internet_route_verification/issues/114).
    <!-- FIXME: It says 59596 as-sets now, the results might shift. -->

- [ ] 4 RPSL USE IN THE WILD:

    > RPSLyzer found 663 syntax errors, 12 invalid as-set names,
    > and 17 invalid route-set names.

    The information is from Parse-Log.
    [#57](https://github.com/SichangHe/internet_route_verification/issues/57).

- [ ] 4 RPSL USE IN THE WILD:

    > Common syntax errors include out-of-place text,
    > such as broken comma-separated lists, misplaced comments,
    > invalid RPSL keywords in import and export rules, or plain typos.

    See Parse-Log and
    [#39](https://github.com/SichangHe/internet_route_verification/discussions/39).

- [ ] 5 VERIFYING AS-PATHS:

    > We ignore 0.06% of routes that are direct exports from
    > the collector’s peer ASes and 0.03% of
    > routes whose AS-paths contain BGP AS-sets.

    In Shell-Evcxr,
    follow the instructions in
    `./route_verification/src/evcxr_examples/as_path_scan_all_ribs.rs`

    [#111](https://github.com/SichangHe/internet_route_verification/issues/111).

- [ ] 5 VERIFYING AS-PATHS:

    > Verifying the 779.3 million routes in all 60 BGP dumps took 2 h 49 m and
    > less than 2 GiB of RAM.

    The information is from RIB-Stats-Time and RIB-Stats-RAM.
    [#157](https://github.com/SichangHe/internet_route_verification/issues/157).

- [ ] 5.1 Special Cases:

    > More than half (6664, 64.4%)
    > of transit ASes specify themselves as an export rule’s filter.

    And:

    > 3090 (29.8%)
    > transit ASes specify a customer AS C in both an import rule’s peering and
    > filter.

    And:

    > A few (46, 0.44%) transit ASes only specify rules for their providers.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.transit_as import main
    main()
    ```

    </details>

    [#134](https://github.com/SichangHe/internet_route_verification/issues/134).

- [ ] 5.2 Verification Results:

    > Figure 2: Route verification status for each AS.

    Follow the instructions in
    `./scripts/scripts/fig/as_all_stacked_area_distin.py`.

- [ ] 5.2 Verification Results:

    > The majority (61,725, 74.4%)
    > of ASes have all imports and exports with identical statuses.
    > We identified 14.2% of ASes with 100% of propagation verified (yellow),
    > 51.6% lacking RPSL information (“unrecorded”, green),
    > 0.34% that only use relaxed filters (blue),
    > and 6.9% with only safelisted relationships (red).

    And:

    > ASes with skipped verifications only constitute 0.03% of ASes.

    And:

    > 25,596 ASes with at least one special-cased import or export
    > (30.9% out of all ASes).

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.as_all_all_some import main
    main()
    ```

    </details>

    The information is in the `\d+ all \w+,` and `\d+ have \w+,` lines.
    [#99](https://github.com/SichangHe/internet_route_verification/issues/99).
    <!-- FIXME: It now says 61747. -->

- [ ] 5.2 Verification Results:

    > Out of the 54.9% of ASes with unrecorded cases,
    > most can be explained by 27.2% of ASes missing aut-num objects and
    > 24.2% of aut-nums with no rules.
    > Excluding ASes with skipped or unrecorded cases,
    > we find more ASes with verified (76.3%) or special-cased (62.5%)
    > routes than ASes with unverified routes (23.1%).

    And Appendix D:

    > the most common unrecorded case is 22,562 ASes not having an aut-num
    > object.
    > The second most common type is for 20,048 ASes that have zero import
    > (or export) rules when verifying an import (or export).

    And:

    > Fewer ASes have rules that refer to ASes with
    > no originating route objects (zero-route ASes, 2706),
    > or set objects (as-set, route-set, peering-set, and filter-set)
    > missing in the IRRs (414).

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.as_unrec_all_breakdown import main
    main()
    ```

    </details>

    [#154](https://github.com/SichangHe/internet_route_verification/issues/154).

- [ ] 5.2 Verification Results:

    > Among these ASes, more incorrectly allow customer route exports (994,
    > “export self”) than imports (325, “import customer”).

    And:

    > most of the special cases are due to uphill propagation with
    > no matching rules (23,298 ASes) or missing route objects (5181 ASes).

    And Appendix D:

    > A small portion (325, 0.4%) of ASes use “import customer”,
    > while more (994, 1.2%) use “export self”.

    And:

    > A significant portion (6.2%) of ASes have missing route objects.
    > ASes that have uphill propagation with
    > no matching RPSL rules occupy a large 28.1% of all ASes,
    > much more than the 12.4% of ASes with unverified routes.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.as_spec_all_all_some import main
    main()
    ```

    </details>

    The information is in the `\d+ have spec_\w+` lines
    ("export self" is called `export_customers`). [#99
    comment](https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2094205769).

- [ ] 5.2 Verification Results:

    > Figure 3: Route verification status for each AS pair.

    Follow the instructions in
    `./scripts/scripts/fig/as_pair_all_stacked_area.py`.

- [ ] 5.2 Verification Results:

    > For imports, we find 96% of AS pairs have a single consistent status;
    > this number is 92% for exports.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.as_pair_all_all_some import main
    main()
    ```

    </details>

    [#99
    comment](https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2240878965).
    <!-- FIXME: The original numbers are for a single RIB.
    The rewritten script gives 91.7 and 92.0. -->

- [ ] 5.2 Verification Results:

    > over half of AS pairs have unverified routes (418,328, 63.0%).

    And:

    > most of them (98.98%)
    > fail verification because the relationship is not declared in the RPSL.

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.fig.dataframes import as_pair_stats_all_df
    PORTS = ("import", "export")
    LEVELS = ("ok", "skip", "unrec", "meh", "err")
    df = as_pair_stats_all_df(
        ["from", "to"]
        + [f"{port}_{tag}" for tag in LEVELS for port in PORTS]
        + ["err_filter"]
    )
    n_as_pair = len(df)
    n_have_err = df[(df["import_err"] > 0) | (df["export_err"] > 0)].__len__()
    n_have_err_filter = df[df["err_filter"] > 0].__len__()
    n_all_err_peering = n_have_err - n_have_err_filter
    print(
        f"{n_have_err} AS pairs have unverified routes {n_have_err * 100 / n_as_pair:.1f}."
    )
    print(
        f"Among them, {n_all_err_peering} AS pairs fail verification because the relationship is not declared in the RPSL ({n_all_err_peering * 100 / n_have_err:.2f})."
    )
    ```

    </details>

    [#94
    comment](https://github.com/SichangHe/internet_route_verification/issues/94#issuecomment-1822005650).
    [#117](https://github.com/SichangHe/internet_route_verification/issues/117).

- [ ] 5.2 Verification Results:

    > Figure 4: Verification status for all hops in BGP routes.

    Follow the instructions in
    `./scripts/scripts/fig/route_all_stacked_area.py`.

- [ ] 5.2 Verification Results:

    > Only 6.6% of routes have the same status across all hops,
    > captured by having a bar of single color (1.6% verified, 3.0% unrecorded,
    > and 1.6% unverified).

    <details>
    <summary>Run this script in Shell-IPython.</summary>

    ```python
    from scripts.stats.route_all_all_some import main
    main()
    ```

    </details>

    The information is in the `all same status,` line and
    the `\d+ all \w+,` lines.
    This script is sequential and takes very long,
    so you may want to open another shell and do other things in the mean time.
    [#99
    comment](https://github.com/SichangHe/internet_route_verification/issues/99#issuecomment-2085328442).

- [ ] 5.2 Verification Results:

    > We also assess the verification status of the first hop in ASpaths…
    > Unfortunately, the results are similar (not shown).

    Follow the instructions in
    `./scripts/scripts/fig/route_first_hop_all_stacked_area.py`.
    The information is in
    the generated `route-first-hop-all-exchange-percentages-stacked-area.pdf`
    file.
    [#141](https://github.com/SichangHe/internet_route_verification/issues/141).

- [ ] Appendix B Nonstandard features:

    > two cases of non-standard but common syntax used by operators
    > (4724 times…)

    [#51](https://github.com/SichangHe/internet_route_verification/discussions/51).
    <!-- TODO:
    We currently cannot reproducing this because it relies on
    previous-version parser's output. -->

- [ ] Appendix B Limitations:

    > We leave the handling of
    > 60 rules whose filters contain AS-path regex with ASN ranges (21 rules)

    Run at `./data/irrs/`
    ([#106](https://github.com/SichangHe/internet_route_verification/issues/106)):

    ```sh
    rg --no-ignore -c '<.*\[\s*AS\d+\s*-\s*AS\d+\s*\].*>'
    ```

    <!-- FIXME: This says 19 instead of 21. The text is outdated. -->

- [ ] Appendix B Limitations:

    > or samepattern unary postfix operators (e.g., ~*, 39 rules)
    > as future work.

    Run at `./data/irrs/`
    ([#113](https://github.com/SichangHe/internet_route_verification/issues/113)):

    ```sh
    rg --no-ignore -c '<.*~.*>'
    ```

- [ ] Appendix B Limitations:

    > we ignore 54 rules with BGP community attributes in their filters.

    In Shell-Evcxr,
    follow the instruction in
    `./route_verification/src/evcxr_examples/community_filter.rs`,
    then evaluate the variable `count`
    ([#158](https://github.com/SichangHe/internet_route_verification/issues/158)).

- [ ] Appendix C:

    <details>
    <summary>Run this script in Shell-Evcxr.</summary>

    ```rust
    let mut line: Line = Line::from_raw("TABLE_DUMP2|1687212014|B|89.149.178.10|3257|103.162.114.0/23|3257 1299 6939 6939 133840 56239 141893|IGP|89.149.178.10|0|10|3257:8794 3257:30052 3257:50001 3257:54900 3257:54901|NAG||".into()).unwrap();
    line.compare.verbosity = Verbosity {
        all_err: true,
        ..Verbosity::minimum_all()
    };
    line.report = Some(line.compare.check_with_relationship(&query, &db));
    line.display();
    ```

    </details>

    [#83](https://github.com/SichangHe/internet_route_verification/issues/83).

- [ ] Appendix D:

    > Figure 5:
    > Breakdown of route verification failures due to unrecorded RPSL objects.

    Follow the instructions in
    `./scripts/scripts/fig/as_unrec_all_stacked_area.py`.

- [ ] Appendix D:

    > Figure 6: Breakdown of special cases per AS.

    Follow the instructions in
    `./scripts/scripts/fig/as_spec_all_stacked_area.py`.
