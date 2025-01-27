# 3D Rendering Engine

The Rendering Engine on the Nintendo DS is responsible for taking in the Vertex And Polygon buffers from the Geometry Engine and constructing frames of video. This can either be displayed as background 0 on the main 2D PPU or captured by the capture unit to be used as a bitmap image.

The Rendering Engine uses a 48-line "Scanline Cache" as opposed to a full framebuffer containing the entire output of a frame when rendering. 