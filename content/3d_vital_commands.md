# Vital 3D Hardware Commands
This chapter will go over the various commands required to be able to display graphics on the 3D hardware.

<a id="3D_CMD_50"></a>
## Swap buffers: Port 0x4000540, Index 0x50, 1 Parameter

Swaps the buffers used between the rendering engine, and the geometry engine.

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0      | Y Sorting for translucent polygons (0=Automatic, 1=Manual)
| 1      | Depth buffering (0 = using Z value, 1 = using W value)
| 2-31   | unused


<a id="3D_CMD_60"></a>
## Set 3D viewport: Port 0x4000580, Index 0x60, 1 Parameter

Sets the region of the screen which the 3D hardware may render to.

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-7    | X1: Left-most X coordinate (0...255)
| 8-15   | Y1: Bottom-most Y coordinate (0...191)
| 16-23  | X2: Right-most X coordinate (0...255)
| 24-31  | Y2: Top-most Y coordinate (0...191)

Notes: 

Due to a misimplementation in the hardware, polygons can still be rendered one pixel beyond (X2, Y1)

Setting the viewport should be reserved to only be done once per frame or once before the beginning of a vertex list.

