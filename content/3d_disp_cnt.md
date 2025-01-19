# 3D Display Control

<style>
tt {
  white-space: pre;
}
</style>

This chapter talks about the registers used to control the 3D display.

<a id="DISP3DCNT"></a>

## DISP3DCNT: 3D display control (0x4000060, R/W)

| Bits    | Description                                             |
|---------|---------------------------------------------------------|
| <tt>0</tt> | Texture mapping enable (0=Disable, 1=Enable)
| <tt>1</tt> | Shading polygon attribute (0=Toon Shading, 1=Highlight Shading)
| <tt>2</tt> | Alpha test (0=Disable, 1=Enable) (see [ALPHA\_TEST\_REF](3d_disp_cnt.md#ALPHA_TEST_REF))
| <tt>3</tt> | Alpha blending (0=Disable, 1=Enable)
| <tt>4</tt> | Anti-aliasing (0=Disable, 1=Enable)
| <tt>5</tt> | Edge marking (0=Disable, 1=Enable) (see EDGE_COLOR)
| <tt>6</tt> | Fog color alpha mode (0=Alpha and color, 1=Only alpha) (see FOG_COLOR)
| <tt>7</tt> | Fog enable (0=Disable, 1=Enable)
| <tt>8-11</tt> | Fog depth shift (<tt>FOG_STEP = 0x400 >> FOG_SHIFT</tt>) (see FOG_OFFSET)
| <tt>12</tt> | Framebuffer RDLINES underflow (0=None, 1=Underflow)
| <tt>13</tt> | Polygon/vertex RAM overflow (0=None, 1=Overflow)
| <tt>14</tt> | Rear plane mode (0=Use clear color, 1=Bitmap)
| <tt>15-31</tt> | Unused

<a id="VIEWPORT"></a>

## VIEWPORT: Set 3D viewport (0x4000580, W)

| Bits    | Description                                             |
|---------|---------------------------------------------------------|
| <tt>0-7</tt> | X1: Left-most X coordinate (0...255)
| <tt>8-15</tt> | Y1: Bottom-most Y coordinate (0...191)
| <tt>16-23</tt> | X2: Right-most X coordinate (0...255)
| <tt>24-31</tt> | Y2: Top-most Y coordinate (0...191)

To fill the screen: X1=0, Y1=0, X2=255, Y2=191

Note that coordinate (0, 0) is the bottom-left corner of the screen, while ht is
the upper-left corner of the screen in the 2D graphics engine.

<a id="DISP_1DOT_DEPTH"></a>

## DISP\_1DOT\_DEPTH: Max depth to render 1-dot polygon (0x4000610, W)

When a polygon is too small or too far away it is reduced to a single pixel on
the screen. This register will determine the cutoff distance that the 3D engine
will use to determine whether to render them or not.

| Bits    | Description                                             |
|---------|---------------------------------------------------------|
| <tt>0-14</tt> | Max W value (12.3 unsigned fixed point)
| <tt>15-31</tt> | Unused

This check can be enabled on a per-polygon basis with bit 13 of POLYGON_ATTR.

The comparison always uses the W coordinate regardless of the buffering mode.

<a id="ALPHA_TEST_REF"></a>

## ALPHA\_TEST\_REF: Alpha test reference value (0x4000340, W)

When alpha test mode is enabled in [DISP3DCNT](3d_disp_cnt.md#DISP3DCNT),
pixels will only be rendered if their alpha value is greater than the value in
`ALPHA_TEST_REF`. Normally, when it is disabled, all pixels are rendered if
their alpha value is greater than zero. A value of 31 will hide all polygons.

This test is done after applying texture and polygon transparency.

| Bits    | Description                                             |
|---------|---------------------------------------------------------|
| <tt>0-4</tt> | Alpha test reference (0..31)
| <tt>5-31</tt> | Unused
