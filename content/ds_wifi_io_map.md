# DS WiFi I/O Map

The following registers are only accessible from the ARM7. They are located at
addresses 0x4808000 to 0x4808FFF. Addresses not included in the following list
aren't used. Also, many of the registers that are used aren't understood.

Important note: 8-bit writes don't work on WiFi registers and RAM, they are
ignored.

<a id="general_registers"></a>
## General registers

| Address | Access | Name              | Description                           |
|---------|--------|-------------------|---------------------------------------|
| 4808000 | R      | `W_ID`            | Chip ID
| 4808004 | R/W    | `W_MODE_RST`      | Reset
| 4808006 | R/W    | `W_MODE_WEP`      | WEP and software modes
| 4808008 | R/W    | `W_TXSTATCNT`     | Beacon status request
| 480800A | R/W    | `W_X_00A`         |
| 4808010 | R/W    | `W_IF`            | WiFi interrupt request flags
| 4808012 | R/W    | `W_IE`            | WiFi interrupt enable
| 4808018 | R/W    | `W_MACADDR_0`     | Hardware MAC address
| 480801A | R/W    | `W_MACADDR_1`     | Hardware MAC address
| 480801C | R/W    | `W_MACADDR_2`     | Hardware MAC address
| 4808020 | R/W    | `W_BSSID_0`       | BSSID
| 4808022 | R/W    | `W_BSSID_1`       | BSSID
| 4808024 | R/W    | `W_BSSID_2`       | BSSID
| 4808028 | R/W    | `W_AID_LOW`       |
| 480802A | R/W    | `W_AID_FULL`      | Association ID
| 480802C | R/W    | `W_TX_RETRYLIMIT` | TX retry limit
| 480802E | R/W    | `W_INTERNAL_02E`  |
| 4808030 | R/W    | `W_RXCNT`         | RX control
| 4808032 | R/W    | `W_WEP_CNT`       | WEP encryption enable
| 4808034 | R?     | `W_INTERNAL_034`  |

<a id="power_registers"></a>
## Power registers

| Address | Access | Name           | Description                              |
|---------|--------|----------------|------------------------------------------|
| 4808036 | R/W    | `W_POWER_US`   |
| 4808038 | R/W    | `W_POWER_TX`   |
| 480803C | R/W    | `W_POWERSTATE` |
| 4808040 | R/W    | `W_POWERFORCE` |
| 4808044 | R      | `W_RANDOM`     |
| 4808048 | R/W    | `W_POWER_048`  |

<a id="receive_control"></a>
## Receive control

| Address | Access | Name              | Description                           |
|---------|--------|-------------------|---------------------------------------|
| 4808050 | R/W    | `W_RXBUF_BEGIN`   |
| 4808052 | R/W    | `W_RXBUF_END`     |
| 4808054 | R      | `W_RXBUF_WRCSR`   |
| 4808056 | R/W    | `W_RXBUF_WR_ADDR` |
| 4808058 | R/W    | `W_RXBUF_RD_ADDR` |
| 480805A | R/W    | `W_RXBUF_READCSR` |
| 480805C | R/W    | `W_RXBUF_COUNT`   |
| 4808060 | R      | `W_RXBUF_RD_DATA` |
| 4808062 | R/W    | `W_RXBUF_GAP`     |
| 4808064 | R/W    | `W_RXBUF_GAPDISP` |

<a id="transmit_control"></a>
## Transmit control

| Address | Access | Name              | Description                           |
|---------|--------|-------------------|---------------------------------------|
| 4808068 | R/W    | `W_TXBUF_WR_ADDR` |
| 480806C | R/W    | `W_TXBUF_COUNT`   |
| 4808070 | W      | `W_TXBUF_WR_DATA` |
| 4808074 | R/W    | `W_TXBUF_GAP`     |
| 4808076 | R/W    | `W_TXBUF_GAPDISP` |
| 4808078 | W      | `W_INTERNAL_078`  |
| 4808080 | R/W    | `W_TXBUF_BEACON`  | Beacon transmit location
| 4808084 | R/W    | `W_TXBUF_TIM`     | Beacon TIM index in frame body
| 4808088 | R/W    | `W_LISTENCOUNT`   | Listen count
| 480808C | R/W    | `W_BEACONINT`     | Beacon interval
| 480808E | R/W    | `W_LISTENINT`     | Listen interval
| 4808090 | R/W    | `W_TXBUF_CMD`     | Multiplay command
| 4808094 | R/W    | `W_TXBUF_REPLY1`  | Multiplay next reply
| 4808098 | R      | `W_TXBUF_REPLY2`  | Multiplay current reply
| 480809C | R/W    | `W_INTERNAL_09C`  |
| 48080A0 | R/W    | `W_TXBUF_LOC1`    |
| 48080A4 | R/W    | `W_TXBUF_LOC2`    |
| 48080A8 | R/W    | `W_TXBUF_LOC3`    |
| 48080AC | W      | `W_TXREQ_RESET`   |
| 48080AE | W      | `W_TXREQ_SET`     |
| 48080B0 | R      | `W_TXREQ_READ`    |
| 48080B4 | W      | `W_TXBUF_RESET`   |
| 48080B6 | R      | `W_TXBUSY`        |
| 48080B8 | R      | `W_TXSTAT`        |
| 48080BA | ?      | `W_INTERNAL_0BA`  |
| 48080BC | R/W    | `W_PREAMBLE`      |
| 48080C0 | R/W    | `W_CMD_TOTALTIME` |
| 48080C4 | R/W    | `W_CMD_REPLYTIME` |
| 48080C8 | ?      | `W_INTERNAL_0C8`  |
| 48080D0 | R/W    | `W_RXFILTER`      |
| 48080D4 | R/W    | `W_CONFIG_0D4`    |
| 48080D8 | R/W    | `W_CONFIG_0D8`    |
| 48080DA | R/W    | `W_RX_LEN_CROP`   |
| 48080E0 | R/W    | `W_RXFILTER2`     |

<a id="wifi_timers"></a>
## WiFi timers

| Address | Access | Name              | Description                           |
|---------|--------|-------------------|---------------------------------------|
| 48080E8 | R/W    | `W_US_COUNTCNT`   | Microsecond counter enable
| 48080EA | R/W    | `W_US_COMPARECNT` | Microsecond compare enable
| 48080EC | R/W    | `W_CONFIG_0EC`    |
| 48080EE | R/W    | `W_CMD_COUNTCNT`  |
| 48080F0 | R/W    | `W_US_COMPARE0`   | Microsecond compare value
| 48080F2 | R/W    | `W_US_COMPARE1`   | Microsecond compare value
| 48080F4 | R/W    | `W_US_COMPARE2`   | Microsecond compare value
| 48080F6 | R/W    | `W_US_COMPARE3`   | Microsecond compare value
| 48080F8 | R/W    | `W_US_COUNT0`     | Microsecond counter
| 48080FA | R/W    | `W_US_COUNT1`     | Microsecond counter
| 48080FC | R/W    | `W_US_COUNT2`     | Microsecond counter
| 48080FE | R/W    | `W_US_COUNT3`     | Microsecond counter
| 4808100 | ?      | `W_INTERNAL_100`  |
| 4808102 | ?      | `W_INTERNAL_102`  |
| 4808104 | ?      | `W_INTERNAL_104`  |
| 4808106 | ?      | `W_INTERNAL_106`  |
| 480810C | R/W    | `W_CONTENTFREE`   |
| 4808110 | R/W    | `W_PRE_BEACON`    |
| 4808118 | R/W    | `W_CMD_COUNT`     |
| 480811C | R/W    | `W_BEACON_COUNT`  |

<a id="configuration_ports"></a>
## Configuration ports

| Address | Access | Name             | Description                            |
|---------|--------|------------------|----------------------------------------|
| 4808120 | R/W    | `W_CONFIG_120`   |
| 4808122 | R/W    | `W_CONFIG_122`   |
| 4808124 | R/W    | `W_CONFIG_124`   |
| 4808126 | ?      | `W_INTERNAL_126` |
| 4808128 | R/W    | `W_CONFIG_128`   |
| 480812A | ?      | `W_INTERNAL_12A` |
| 4808130 | R/W    | `W_CONFIG_130`   |
| 4808132 | R/W    | `W_CONFIG_132`   |
| 4808134 | R/W    | `W_POST_BEACON`  |
| 4808140 | R/W    | `W_CONFIG_140`   |
| 4808142 | R/W    | `W_CONFIG_142`   |
| 4808144 | R/W    | `W_CONFIG_144`   |
| 4808146 | R/W    | `W_CONFIG_146`   |
| 4808148 | R/W    | `W_CONFIG_148`   |
| 480814A | R/W    | `W_CONFIG_14A`   |
| 480814C | R/W    | `W_CONFIG_14C`   |
| 4808150 | R/W    | `W_CONFIG_150`   |
| 4808154 | R/W    | `W_CONFIG_154`   |

<a id="baseband_chip"></a>
## Baseband chip

| Address | Access | Name            | Description                             |
|---------|--------|-----------------|-----------------------------------------|
| 4808158 | W      | `W_BB_CNT`      | BB access control
| 480815A | W      | `W_BB_WRITE`    | Byte to write to BB
| 480815C | R      | `W_BB_READ`     | Byte read from BB
| 480815E | R      | `W_BB_BUSY`     | BB access busy Flag
| 4808160 | R/W    | `W_BB_MODE`     | BB access mode
| 4808168 | R/W    | `W_BB_POWER`    | BB access powerdown

<a id="internal_registers"></a>
## Internal registers

| Address | Access | Name             | Description                            |
|---------|--------|------------------|----------------------------------------|
| 480816A | ?      | `W_INTERNAL_16A` |
| 4808170 | ?      | `W_INTERNAL_170` |
| 4808172 | ?      | `W_INTERNAL_172` |
| 4808174 | ?      | `W_INTERNAL_174` |
| 4808176 | ?      | `W_INTERNAL_176` |
| 4808178 | W      | `W_INTERNAL_178` |

<a id="rf_chip"></a>
## RF chip

| Address | Access | Name             | Description                            |
|---------|--------|------------------|----------------------------------------|
| 480817C | R/W    | `W_RF_DATA2`     |
| 480817E | R/W    | `W_RF_DATA1`     |
| 4808180 | R      | `W_RF_BUSY`      |
| 4808184 | R/W    | `W_RF_CNT`       |
| 4808190 | R/W    | `W_INTERNAL_190` |
| 4808194 | R/W    | `W_TX_HDR_CNT`   |
| 4808198 | R/W    | `W_INTERNAL_198` |
| 480819C | R      | `W_RF_PINS`      |
| 48081A0 | R/W    | `W_X_1A0`        |
| 48081A2 | R/W    | `W_X_1A2`        |
| 48081A4 | R/W    | `W_X_1A4`        |

<a id="wifi_statistics"></a>
## WiFi statistics

| Address | Access | Name              | Description                           |
|---------|--------|-------------------|---------------------------------------|
| 48081A8 | R      | `W_RXSTAT_INC_IF` | Stats increment flags
| 48081AA | R/W    | `W_RXSTAT_INC_IE` | Stats increment IRQ enable
| 48081AC | R      | `W_RXSTAT_OVF_IF` | Stats half-overflow flags
| 48081AE | R/W    | `W_RXSTAT_OVF_IE` | Stats half-overflow IRQ enable
| 48081B0 | R/W    | `W_RXSTAT`        |
| 48081B2 | R/W    | `W_RXSTAT`        |
| 48081B4 | R/W    | `W_RXSTAT`        |
| 48081B6 | R/W    | `W_RXSTAT`        |
| 48081B8 | R/W    | `W_RXSTAT`        |
| 48081BA | R/W    | `W_RXSTAT`        |
| 48081BC | R/W    | `W_RXSTAT`        |
| 48081BE | R/W    | `W_RXSTAT`        |
| 48081C0 | R/W    | `W_TX_ERR_COUNT`  | TX error count
| 48081C4 | R      | `W_RX_COUNT`      |
| 48081D0 | R/W    | `W_CMD_STAT`      |
| 48081D2 | R/W    | `W_CMD_STAT`      |
| 48081D4 | R/W    | `W_CMD_STAT`      |
| 48081D6 | R/W    | `W_CMD_STAT`      |
| 48081D8 | R/W    | `W_CMD_STAT`      |
| 48081DA | R/W    | `W_CMD_STAT`      |
| 48081DC | R/W    | `W_CMD_STAT`      |
| 48081DE | R/W    | `W_CMD_STAT`      |

<a id="internal_diagnostics"></a>
## Internal diagnostics

| Address | Access | Name             | Description                            |
|---------|--------|------------------|----------------------------------------|
| 48081F0 | R/W    | `W_INTERNAL_1F0` |
| 4808204 | ?      | `W_INTERNAL_204` |
| 4808208 | ?      | `W_INTERNAL_208` |
| 480820C | W      | `W_INTERNAL_20C` |
| 4808210 | R      | `W_TX_SEQNO`     |
| 4808214 | R      | `W_RF_STATUS`    |
| 480821C | W      | `W_IF_SET`       | Set bits in `W_IF` to force interrupts.
| 4808220 | R/W    | `W_RAM_DISABLE`  | WiFi RAM control
| 4808224 | R/W    | `W_INTERNAL_224` |
| 4808228 | W      | `W_X_228`        |
| 4808230 | R/W    | `W_INTERNAL_230` |
| 4808234 | R/W    | `W_INTERNAL_234` |
| 4808238 | R/W    | `W_INTERNAL_238` |
| 480823C | ?      | `W_INTERNAL_23C` |
| 4808244 | R/W    | `W_X_244`        |
| 4808248 | R/W    | `W_INTERNAL_248` |
| 480824C | R      | `W_INTERNAL_24C` |
| 480824E | R      | `W_INTERNAL_24E` |
| 4808250 | R      | `W_INTERNAL_250` |
| 4808254 | ?      | `W_CONFIG_254`   |
| 4808258 | ?      | `W_INTERNAL_258` |
| 480825C | ?      | `W_INTERNAL_25C` |
| 4808260 | ?      | `W_INTERNAL_260` |
| 4808264 | R      | `W_INTERNAL_264` |
| 4808268 | R      | `W_RXTX_ADDR`    |
| 4808270 | R      | `W_INTERNAL_270` |
| 4808274 | ?      | `W_INTERNAL_274` |
| 4808278 | R/W    | `W_INTERNAL_278` |
| 480827C | ?      | `W_INTERNAL_27C` |
| 4808290 | (R/W)  | `W_X_290`        |
| 4808298 | W      | `W_INTERNAL_298` |
| 48082A0 | R/W    | `W_INTERNAL_2A0` |
| 48082A2 | R      | `W_INTERNAL_2A2` |
| 48082A4 | R      | `W_INTERNAL_2A4` |
| 48082A8 | W      | `W_INTERNAL_2A8` |
| 48082AC | ?      | `W_INTERNAL_2AC` |
| 48082B0 | W      | `W_INTERNAL_2B0` |
| 48082B4 | R/W    | `W_INTERNAL_2B4` |
| 48082B8 | ?      | `W_INTERNAL_2B8` |
| 48082C0 | R/W    | `W_INTERNAL_2C0` |
| 48082C4 | R      | `W_INTERNAL_2C4` |
| 48082C8 | R      | `W_INTERNAL_2C8` |
| 48082CC | R      | `W_INTERNAL_2CC` |
| 48082D0 | ?      | `W_INTERNAL_2D0` |
| 48082F0 | R/W    | `W_INTERNAL_2F0` |
| 48082F2 | R/W    | `W_INTERNAL_2F2` |
| 48082F4 | R/W    | `W_INTERNAL_2F4` |
| 48082F6 | R/W    | `W_INTERNAL_2F6` |
