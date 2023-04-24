# Parse RPSL Policy

## Test

Run at `./`:

```shell
pytest
```

To test against `ripe.db` using `rpsl_policy/tests/mp_import_w_db.py`,
put the database file at `data/ripe.db`:

```bash
python3 -m rpsl_policy.tests.mp_import_w_db
```

Similarly, to test with `rpsl_policy/tests/mp_export_w_db.py`,

```bash
python3 -m rpsl_policy.tests.mp_export_w_db
```

Similarly, to test with `rpsl_policy/tests/mp_peering_w_db.py`,

```bash
python3 -m rpsl_policy.tests.mp_peering_w_db
```
