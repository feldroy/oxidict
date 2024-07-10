# CHANGELOG

## 0.1.0

This now works:

```
>>> from oxidict import OxiDict
>>> o = OxiDict()
>>> o.insert("key1", "value1")
>>> print(o.get("key1"))
value1
>>> print(o)
{"key1": "value1"}
```
