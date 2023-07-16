# Parse RPSL Policy

WIP

## Produce a parsed dump using both lexer and parser

- Put the database file at `data/ripe.db`.
- Make sure `pypy3` is in the `PATH`.
- Make sure you have `pyparsing` installed.
- Prepend this directory to your `PYTHONPATH`.
- Maybe you want to enable logging:

    ```sh
    export RUST_LOG=route_verification=trace,route_verification_irr=trace,route_verification_lex=trace,route_verification_parse=trace
    ```

- Run at `route_verification/`:

    ```sh
    cargo r --release -- parse ../data/ripe.db ../parsed
    ```

    The parsed dump will be distributed in `parsed/`.

## Produce a spread parsed dump from both priority and backup registries

### Obtain IRR data

Download from all FTP servers on [IRR List of Routing
Registries](https://www.irr.net/docs/list.html).

Download priority registries to `data/irrs/priority/`:

<ftp://ftp.afrinic.net/pub/dbase/>
<ftp://ftp.altdb.net/pub/altdb/>
<ftp://ftp.apnic.net/pub/apnic/whois/>
<ftp://ftp.arin.net/pub/rr/>
<ftp://irr.bboi.net/>
<https://whois.canarie.ca/dbase/>
<ftp://irr-mirror.idnic.net/>
<ftp://ftp.nic.ad.jp/jpirr/>
<ftp://irr.lacnic.net/>
<ftp://ftp.nestegg.net/irr>
<ftp://rr1.ntt.net/nttcomRR/>
<ftp://ftp.panix.com/pub/rrdb>
<ftp://ftp.ripe.net/ripe/dbase/>

Download backup registries to `data/irrs/backup/`:

<ftp://ftp.radb.net/radb/dbase/>

Decompress all files.

### Run the parser with `parse_priority`

Run at `route_verification/`:

```sh
cargo r --release -- parse_priority ../data/irrs/priority/ ../data/irrs/backup/ ../parsed_all/
```

The above command parses all IRR DB files in `data/irrs/priority/` and
`data/irrs/backup/`,
overrides any duplicated information with the version from the former,
and writes the result to multiple JSON files in `parsed_all/`.

## Running interactively in Jupyter Notebook

- Finish the previous section. Your parsed dump should be cached in `parsed/`.
- [Install Evcxr Jupyter Kernel](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md).
- Open the notebook at `./`, and try out `parse_test.ipynb`.

## Produce a lexed dump using lexer

If the database file is at `data/ripe.db`, for example:

```sh
pypy3 -m rpsl_lexer.dump data/ripe.db > dump.json
```

In the command above, we use PyPy for faster performance,
and pipe the dumped JSON to `dump.json`.

## Test lexer

Run at `./`:

```shell
pytest
```

### Test lexer against `ripe.db`

To test against `ripe.db` using `rpsl_lexer/tests/mp_import_w_db.py`,
put the database file at `data/ripe.db`:

```bash
python3 -m rpsl_lexer.tests.mp_import_w_db
```

Similarly, to test with `rpsl_lexer/tests/mp_export_w_db.py`,
or one of the other lexers:

```bash
python3 -m rpsl_lexer.tests.mp_export_w_db
python3 -m rpsl_lexer.tests.mp_peering_w_db
python3 -m rpsl_lexer.tests.mp_filter_w_db
python3 -m rpsl_lexer.tests.action_w_db
python3 -m rpsl_lexer.tests.import_w_db
python3 -m rpsl_lexer.tests.export_w_db
```

## Test parser and comparator

Run at `route_verification/`:

```sh
cargo t --workspace
```
