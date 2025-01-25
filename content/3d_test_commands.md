# 3D Test Commands

These Commands test various parameters against desired results by the user. Each command requires that the "test busy" bit of the ``GXSTAT`` register is cleared before reading the result.

<a id="3D_CMD_70"></a>
## Test if Cuboid Intersects View Volume: Port 0x040005C0, Index 0x70, 3 Parameters

The result of this command can be read from bit 1 of the ``GXSTAT`` register and indicates if any part of the specified cuboid intesects the view volume. A value of 1 indicates that the cuboid would have been visible if drawn.

Parameter definition:

| Parameter | Bit(s) | Description |
|-----------|--------|-------------|
| 1         | 0-15   | Origin X coordinate
| 1         | 16-31  | Origin Y coordinate
| 2         | 0-15   | Origin Z coordinate
| 2         | 16-31  | Width (X-Offset)
| 3         | 0-15   | Height (Y-Offset)
| 3         | 16-31  | Depth (Z-Offset)

all parameters are in the same format as 16 bit vertex coordinates. As in, 1bit sign + 3 bit integer + 12 bit fraction.

<a id="3D_CMD_71"></a>
## Set Coordinates for Position Test: Port 0x040005C4, Index 0x71, 2 Parameters

Takes the vector ``(x,y,z,1)`` and multiplies by the positional and projection matrix stacks. Result can be read from the memory region at ``0x04000620..=0x0400062F`` where each word corresponds to a coordinate of the resulting vector. In format 1bit sign + 19 bit integer + 12 bit fraction.

Parameter Definition:

| Parameter | Bit(s) | Description |
|-----------|--------|-------------|
| 1         | 0-15   | X coordinate
| 1         | 16-31  | Y coordinate
| 2         | 0-15   | Z coordinate
| 2         | 16-31  | Unused

all parameters are in the same format as 16 bit vertex coordinates. As in, 1bit sign + 3 bit integer + 12 bit fraction.

Notes: This command should not be issued while a vertex list is being constructed. As any vertex position commands that inherit the coordinates of the previous vertex will instead inherit the coordinates set by this command.

<a id="3D_CMD_72"></a>
## Set Directional vector for Direction Test: Port 0x040005C8, Index 0x72, 1 Parameter

Takes the vector ``(x,y,z,0)`` and multiplies by the directional matrix stack. (according to no$ also requires matrix mode 2?) Result can be read from the memory region at ``0x04000630..=0x04000635`` where each 2byte half word corresponds to a coordinate of the resulting vector. In format 4bit sign + 12 bit fraction. (sign is either ``0b0000`` or ``0b1111``)

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-9    | X coordinate
| 10-19  | Y coordinate
| 20-29  | Z coordinate
| 30-31  | Unused

all parameters are in the same format as those for other light direction commands, as in: 1bit sign + 9 bit fraction.