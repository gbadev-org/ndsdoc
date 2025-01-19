# 3D Display Control

This chapter talks about the registers used to control the 3D display.

<a id="DISP3DCNT"></a>
## DISP3DCNT: 3D display control (0x4000060, R/W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0      | Texture mapping enable (0=Disable, 1=Enable)
| 1      | Shading polygon attribute (0=Toon Shading, 1=Highlight Shading)
| 2      | Alpha test (0=Disable, 1=Enable) (see [ALPHA\_TEST\_REF](3d_disp_cnt.md#ALPHA_TEST_REF))
| 3      | Alpha blending (0=Disable, 1=Enable)
| 4      | Anti-aliasing (0=Disable, 1=Enable)
| 5      | Edge marking (0=Disable, 1=Enable) (see `EDGE_COLOR`)
| 6      | Fog color alpha mode (0=Alpha and color, 1=Only alpha) (see `FOG_COLOR`)
| 7      | Fog enable (0=Disable, 1=Enable)
| 8-11   | Fog depth shift (`FOG_STEP = 0x400 >> FOG_SHIFT`) (see `FOG_OFFSET`)
| 12     | Framebuffer RDLINES underflow (0=None, 1=Underflow)
| 13     | Polygon/vertex RAM overflow (0=None, 1=Overflow)
| 14     | Rear plane mode (0=Use clear color, 1=Bitmap)
| 15-31  | Unused

<a id="VIEWPORT"></a>
## VIEWPORT: Set 3D viewport (0x4000580, W)

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-7    | X1: Left-most X coordinate (0...255)
| 8-15   | Y1: Bottom-most Y coordinate (0...191)
| 16-23  | X2: Right-most X coordinate (0...255)
| 24-31  | Y2: Top-most Y coordinate (0...191)

To fill the screen: X1=0, Y1=0, X2=255, Y2=191

Note that coordinate (0, 0) is the bottom-left corner of the screen, while ht is
the upper-left corner of the screen in the 2D graphics engine.

<a id="DISP_1DOT_DEPTH"></a>
## DISP\_1DOT\_DEPTH: 1-dot polygon render depth (0x4000610, W)

When a polygon is too small or too far away it is reduced to a single pixel on
the screen. This register will determine the cutoff distance that the 3D engine
will use to determine whether to render them or not.

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-14   | Max W value (12.3 unsigned fixed point)
| 15-31  | Unused

This check can be enabled on a per-polygon basis with bit 13 of `POLYGON_ATTR`.

The comparison always uses the W coordinate regardless of the buffering mode.

<a id="ALPHA_TEST_REF"></a>
## ALPHA\_TEST\_REF: Alpha test reference (0x4000340, W)

When alpha test mode is enabled in [DISP3DCNT](3d_disp_cnt.md#DISP3DCNT),
pixels will only be rendered if their alpha value is greater than the value in
`ALPHA_TEST_REF`. Normally, when it is disabled, all pixels are rendered if
their alpha value is greater than zero. A value of 31 will hide all polygons.

This test is done after applying texture and polygon transparency.

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-4    | Alpha test reference (0...31)
| 5-31   | Unused
