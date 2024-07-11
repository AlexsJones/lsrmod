lsrmod
======

Rust based implementation of `lsmod`

Also prints in JSON for automation and scripting usefulness.

Install
======

```bash
cargo install lsrmod
```

Example
=======

```
lsrmod --json | jq . | head -n25
[
  {
    "name": "tls",
    "memory": "151552",
    "instances": 0,
    "depends_on": "-",
    "state": "Live",
    "memory_offset": "0x0000000000000000"
  },
  {
    "name": "r8153_ecm",
    "memory": "12288",
    "instances": 0,
    "depends_on": "-",
    "state": "Live",
    "memory_offset": "0x0000000000000000"
  },
  {
    "name": "cdc_ether",
    "memory": "24576",
    "instances": 1,
    "depends_on": "r8153_ecm,",
    "state": "Live",
    "memory_offset": "0x0000000000000000"
  },

```