# 3D Light Commands

These are the commands which specifies the lighting conditions for any upcoming polygons and verticies.

<a id="3D_CMD_32"></a>
## Set Lights Directional Vector: Port 0x040004C8, Index 0x32, 1 Parameter

Sets the direction a given light points in.

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-9    | X coordinate
| 10-19  | Y coordinate
| 20-29  | Z coordinate
| 30-31  | Light Index (0..=3)

all coordinate parameters are in the same format, as in: 1bit sign + 9 bit fraction.

<a id="3D_CMD_33"></a>
## Set Lights Color: Port 0x040004CC, Index 0x33, 1 Parameter

Sets the color of a given light.

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-4    | Red
| 5-9    | Green
| 10-14  | Blue
| 15-29  | Unused
| 30-31  | Light Index (0..=3)

<a id="3D_CMD_34"></a>
## Set Shininess table: Port 0x040004D0, Index 0x34, 32 Parameters

Sets the contents of a 128-byte shininess table used for specular reflections. transferred 4 entries (bytes) at a time. 0 = least shiny, 255 = most shiny. 

Notes: When the shininess table is disabled, the Rendering Engine will act as if the table is filled with linearly increasing entries from the minimum to the maximum.