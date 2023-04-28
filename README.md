# Parse RPSL Policy

## Test

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
