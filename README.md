# Parse RPSL Policy

## Produce a dump using lexer

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

### Test against `ripe.db`

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
