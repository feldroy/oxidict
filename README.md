# OxiDict

Rust's HashMap brought over to Python with PyO3.

```bash
python3.12 -m venv .venv
source .venv/bin/activate
pip install -U pip maturin
maturin develop
```

In the Python shell:

```
>>> from oxidict import OxiDict
>>> o = OxiDict()
>>> o.insert("key1", "value1")
>>> print(o.get("key1"))
value1
>>> print(o)
{"key1": "value1"}
```