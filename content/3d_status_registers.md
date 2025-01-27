# 3D Display Control

This chapter talks about the status registers of the 3D hardware.

<a id="GXSTAT"></a>
## GXSTAT: Geometry Engine Status Register (0x04000600, mixed R/W)

| Bit(s) | Description        |
|--------|--------------------|
| 0      | Test functions Busy
| 1      | Box Test Result
| 2-7    | unused
| 8-12   | Position & Directional Matrix Stack Pointer
| 13     | Projection Matrix Stack Pointer
| 14     | Matrix Stack Busy
| 15     | Matrix Stack Overflow/Undeflow
| 16-24  | Number of 40-bit entries in Command FIFO
| 25     | FIFO is less than half full
| 26     | FIFO is empty
| 27     | General Busy Flag
| 28-29  | unused
| 30-31  | Command FIFO IRQ (0 = Never, 1 = Less than half full, 2 = Empty, 3 = Reserved)


## RAM statistics: Polygon List & Vertex RAM Count (0x04000604, R)

| Bit(s) | Description        |
|--------|--------------------|
| 0-11   | Number of Polygons currently in Polygon List RAM
| 12-15  | unused
| 16-28  | Number of Verticies currently in Vertex RAM
| 29-31  | unused

Note: Once a buffer swap has been sent, the counters are reset 10 cycles after V-Blank.

## Performance Statistics (0x04000320, R)

| Bit(s) | Description        |
|--------|--------------------|
| 0-5    | Minimum number of buffered lines during last frame - 2
| 6-31   | unused

