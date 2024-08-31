# RPSLyzer: Parse RPSL Policies and Verify BGP Routes

RPSLyzer provides libraries and examples to
parse the Routing Policy Specification Language (RPSL)
from Internet Routing Registries (IRRs)
and verify interdomain routes from Border Gateway Protocol (BGP)
table dumps against them.
The focus of parsing is on the RPSL objects and attributes related to
routing policies, centering on the `aut-num` objects.
For the verification,
we simply walk through the AS-path in each BGP route and
interpret the policy in each AS's `aut-num` object with the context of
the route.

After parsing the RPSL, we expose an intermediate representation (IR)
in a JSON-compatible nested Rust data structure.
You may thus access this IR from other languages via the serialized JSON.

## Usage

As a user, you need to install the required tools, set up the environments,
and use [the `route_verification` Rust package][docs_route_verification]
directly.
Please refer to `./ARTIFACTS.md` for the our data acquisition, parsing,
verification, and analysis workflow.
Note: `./ARTIFACTS.md` is a work in progress (WIP);
please see [#164][issue164].

## Repository structure

- We annotate the RPSL-related Request for Comments (RFCs) at `./rfcs/`.
    Please check these documents for the RPSL-related terminologies we use and
    the limitations of RPSLyzer.

- The RPSL lexer at `./rpsl_lexer/` uses PyParsing and Python,
    and is compatible with PyPy.
    We publish it
    [on PyPI as `rpsl-lexer`](https://pypi.org/project/rpsl-lexer/).

    `rpsl-lexer` tokenizes ("lexes")
    specific RPSL syntaxes into an abstract syntax tree (AST),
    especially `mp-import`, `mp-export`,
    and the `<peering>` and `<filter>` portions they contain.
    We primarily call this library via UNIX pipes from the Rust library that
    lexes the RPSL (`route_verification_lex`).

    We chose to tokenize with PyParsing to leverage the power of
    parsing expression grammar (PEG) for recursively-defined expressions.

- The RPSL parser, verification logic, and read-evaluate-print loop (REPL)
    shell script examples at `./route_verification/` are a series of
    Rust crates (Rust packages) and scripts.
    All main library crates are re-exported in
    [the `route_verification` crate][docs_route_verification].

    All re-exported crates have `route_verification_` prepended to
    their path names. Among these crates:

    - `route_verification_ir` defines the IR and the relevant procedures.
    - `route_verification_lex` lexes the RPSL source code into the AST.
        It requires setting up `rpsl-lexer` for lexing (see above).
    - `route_verification_parse` parses the RPSL source code into the IR.
        It leverages `route_verification_lex` for lexing,
        then parses the AST into the IR.
    - `route_verification_irr` parses the RPSL source code from IRRs and
        merges them into a single IR.
    - `route_verification_as_rel` parses Center for
        Applied Internet Data Analysis (CAIDA)'s
        [AS-relationship
        dataset](https://data.caida.org/datasets/2013-asrank-data-supplement/)
        to augment the verification.
    - `route_verification_bgp` optimizes the IR for querying and
        verifies BGP routes against the IR.
        It can optionally merge in information from
        the AS-relationship dataset (via pseudo `as-set`s),
        and apply special cases based on
        these relationships during verification.

        Additionally,
        `route_verification` provides a command-line interface (CLI)
        to parse IRRs and to test run verification.
        The REPL scripts are in
        the module at `./route_verification/src/evcxr_examples.rs`.
        `route_verification_rib_stats` is a main script to
        generate statistics for all BGP table dumps at a directory, on the AS,
        AS-pair, and route levels.

        We chose Rust for the IR, parser,
        and verification logic for its strongly-typed `enum`eration and
        satisfactory performance at CPU-bound tasks.

- The scripts we use to analyze and
    visualize the results at `./scripts/` serve as examples for these tasks.
    Most of them are in Python and
    leverages common Python data analysis libraries,
    though one of the CPU-bound scripts is in Rust.

- `./ARTIFACTS.md` and `./ALIASES.md` explain how to run the scripts and
    the meanings of common variable names.
    These are still a WIP; please see [#164][issue164].

In addition to the code and short documentation,
this repository's
[Issues](https://github.com/SichangHe/internet_route_verification/issues)
contain detailed discussions and development records about this project.
We recommend using GitHub's search to find relevant information in
case you encounter issues.

## Build tools

We leverage user-friendly, reproducible, and automatic build tools everywhere.
All Rust crates use Cargo,
and Python libraries and scripts use Rye. Please see `./ARTIFACTS.md` for
more information.

## Debugging Rust

- Enable logging:

    ```sh
    export RUST_LOG=route_verification=trace
    ```

- Enable backtrace (stack trace) in error messages:

    ```sh
    export RUST_BACKTRACE=1
    ```

## Testing

Please see the GitHub Actions at `.github/workflows/` for up-to-date tests.

## Maintenance status

Maintenance mode. We are not actively developing this project.
Only bug-fix contributions will be considered.
Please fork and modify as needed.

## Related projects

[Internet Route Verification
Server](https://github.com/SichangHe/internet_route_verification_server)
is an abandoned attempt to store the IR and the verification results in
a PostgreSQL database and serve predefined queries via REST APIs.
We abandoned it due to its limited utility and the tebibytes of
disk space needed.

## Paper

This is the code and Issue repository corresponding to the paper: *RPSLyzer:
Characterization and Verification of Policies in Internet Routing Registries*.
This paper was accepted at ACM IMC'24.
We are working on a camera-ready version, and will link a preprint here.

[docs_route_verification]: https://docs.rs/route_verification/latest/route_verification/
[issue164]: https://github.com/SichangHe/internet_route_verification/pull/164
