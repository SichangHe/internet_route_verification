# Parse RPSL Policy

WIP

## Obtain IRR data

Download from all FTP servers on [IRR List of Routing
Registries](https://www.irr.net/docs/list.html).

ftp://ftp.radb.net/radb/dbase
ftp://ftp.afrinic.net/pub/dbase/
ftp://ftp.apnic.net/pub/apnic/whois/
ftp://irr-mirror.idnic.net/
ftp://irr.lacnic.net/
ftp://ftp.ripe.net/ripe/dbase/

## Produce a parsed dump using both lexer and parser

- Put the database file at `data/ripe.db`.
- Make sure `pypy3` is in the `PATH`.
- Make sure you have `pyparsing` installed.
- Prepend this directory to your `PYTHONPATH`.
- Maybe you want to enable logging:

    ```sh
    export RUST_LOG=route_policy_cmp=trace
    ```

- Run at `route_policy_cmp/`:

    ```sh
    cargo r --release -- parse ../data/ripe.db ../parsed
    ```

    The parsed dump will be distributed in `parsed/`.

## Running interactively in Jupyter Notebook

- Finish the previous section. Your parsed dump should be cached in `parsed/`.
- [Install Evcxr Jupyter Kernel](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md).
- Open the notebook at `./`, and try out `parse_test.ipynb`.

## Produce a lexed dump using lexer

If the database file is at `data/ripe.db`, for example:

```sh
pypy3 -m rpsl_policy.dump data/ripe.db > dump.json
```

In the command above, we use PyPy for faster performance,
and pipe the dumped JSON to `dump.json`.

## Test lexer

Run at `./`:

```shell
pytest
```

### Test lexer against `ripe.db`

To test against `ripe.db` using `rpsl_policy/tests/mp_import_w_db.py`,
put the database file at `data/ripe.db`:

```bash
python3 -m rpsl_policy.tests.mp_import_w_db
```

Similarly, to test with `rpsl_policy/tests/mp_export_w_db.py`,
or one of the other lexers:

```bash
python3 -m rpsl_policy.tests.mp_export_w_db
python3 -m rpsl_policy.tests.mp_peering_w_db
python3 -m rpsl_policy.tests.mp_filter_w_db
python3 -m rpsl_policy.tests.action_w_db
python3 -m rpsl_policy.tests.import_w_db
python3 -m rpsl_policy.tests.export_w_db
```

## Test parser and comparator

Run at `route_policy_cmp/`:

```sh
cargo t
```
