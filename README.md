lsrmod
======

Rust based implementation of `lsmod`

Also prints in JSON for automation and scripting usefulness.

```
e.g., lsrmod --number-of-lines=10
+----------------------+-----------+-----------+----------------------+-------+--------------------+
| Name                 | Memory(b) | Instances | Depends on           | State | Memory Offset      |
| ipheth               | 20480     | 0         | -                    | Live  | 0x0000000000000000 |
| apple_mfi_fastcharge | 16384     | 0         | -                    | Live  | 0x0000000000000000 |
| tls                  | 151552    | 0         | -                    | Live  | 0x0000000000000000 |
| r8153_ecm            | 12288     | 0         | -                    | Live  | 0x0000000000000000 |
| cdc_ether            | 24576     | 1         | r8153_ecm,           | Live  | 0x0000000000000000 |
| usbnet               | 61440     | 2         | r8153_ecm,cdc_ether, | Live  | 0x0000000000000000 |
| r8152                | 143360    | 1         | r8153_ecm,           | Live  | 0x0000000000000000 |
| mii                  | 20480     | 2         | usbnet,r8152,        | Live  | 0x0000000000000000 |
| snd_usb_audio        | 499712    | 0         | -                    | Live  | 0x0000000000000000 |
| snd_usbmidi_lib      | 53248     | 1         | snd_usb_audio,       | Live  | 0x0000000000000000 |
+----------------------+-----------+-----------+----------------------+-------+--------------------+

```

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