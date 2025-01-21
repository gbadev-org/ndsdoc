# DS WiFi General Registers

<a id="W_ID"></a>
## W\_ID: Chip ID (0x4808000, R)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   | Chip ID

This register returns 0x1440 in NDS, 0xC340 in NDS lite.

<a id="W_MODE_RST"></a>
## W\_MODE\_RST: Reset (0x4808004, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   | ???

<a id="W_MODE_WEP"></a>
## W\_MODE\_WEP: WEP mode (0x4808006, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-2    | Software mode?
| 3-5    | WEP key size

<a id="W_TXSTATCNT"></a>
## W\_TXSTATCNT: Beacons status register (0x4808008, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-11   | ???
| 12     |
| 13     |
| 14     |
| 15     |

<a id="W_X_00A"></a>
## W\_X\_00A: Unknown (0x480800A, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   | ???

<a id="W_IF"></a>
## W\_IF: Interrupt request flags (0x4808010, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   | See [W\_IE](ds_wifi_general_regs.md#W_IE))

<a id="W_IE"></a>
## W\_IE: Interrupt enable (0x4808012, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0      |
| 1      |
| 2      |
| 3      |
| 4      |
| 5      |
| 6      |
| 7      |
| 8      |
| 9      |
| 10     | Unused
| 11     |
| 12     |
| 13     |
| 14     |
| 15     |

<a id="W_MACADDR_0"></a>
## W\_MACADDR\_0: Hardware MAC address (0x4808018, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_MACADDR_1"></a>
## W\_MACADDR\_1: Hardware MAC address (0x480801A, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_MACADDR_2"></a>
## W\_MACADDR\_2: Hardware MAC address (0x480801C, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_BSSID_0"></a>
## W\_BSSID\_0: BSSID (0x4808020, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_BSSID_1"></a>
## W\_BSSID\_1: BSSID (0x4808022, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_BSSID_2"></a>
## W\_BSSID\_2: BSSID (0x4808024, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_AID_LOW"></a>
## W\_AID\_LOW: Unknown (0x4808028, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_AID_FULL"></a>
## W\_AID\_FULL: Association ID (0x480802A, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_TX_RETRYLIMIT"></a>
## W\_TX\_RETRYLIMIT: TX retry limit (0x480802C, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_INTERNAL_02E"></a>
## W\_INTERNAL\_02E: Unknown (0x480802E, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   | Unknown

<a id="W_RXCNT"></a>
## W\_RXCNT: RX control (0x4808030, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_WEP_CNT"></a>
## W\_WEP\_CNT: WEP encryption enable (0x4808032, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   |

<a id="W_INTERNAL_034"></a>
## W\_INTERNAL\_034: Unknown (0x4808034, R?)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-15   | Unknown
