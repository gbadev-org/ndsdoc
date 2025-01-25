# 3D Geometry Engine

The Geometry Engine on the Nintendo DS is the hardware responsible for taking in 3D Graphics Commands and coverting them to verticies and polygons which can be rendered by the Rendering Engine. It's jobs include performing various matrix multiplications involving vertex positions, vertex colors, light vectors, etc.

# Readable Matricies

The Geometry provides access to the final directional matrix and screen space transformation matrix. (as in, position matricies * projection matrix) Reading these requires the geometry engine to be disabled via bit 27 in the ``GXSTAT`` register.

## Clip Matrix (0x4000640..=0x400067F, 16 words, R)

Read the 4x4 "Clip matrix" with cells going from row 0 column 0, row by row.

## Read Directional Matrix (0x4000680..=0x40006A3, 9 words, R)

Read the "top left" 3x3 segment of the directional matrix with cells going from row 0 column 0, row by row.

# 3D Command FIFO
The Geometry engine is heavily based upon a Command FIFO and PIPE used to transfer vertex lists, bind textures, set viewport, modify matricies, etc. The FIFO can be directly accessed by writing command numbers and parameters to memory location ``0x4000400`` or indirectly by writing parameters to the corresponding "Command Ports" at various memory locations from``0x4000440..=0x40005FF``.

## Using the FIFO indirectly
The simpler and more user friendly method of interacting with the FIFO is via the "Command Ports", where parameters are written to the corresponding memory address for a given command. The command + parameter combo is automatically sent down the FIFO to the 3D hardware once every parameter has been sent. For commands that don't have any parameters one may simply write any value to the port. 

## Using the FIFO directly
When using the FIFO directly, commands are sent by first writing a "Command word" containing up to 4 packed command indicies. Followed by writing the parameters for each command in order. If the last command does not have a parameter, 0 must be written as a dummy parameter in order for the hardware to accept a new "command word". When trying to specify invalid command indicies they will be treated the same as command index 0. (no command, no parameters) This way of using the fifo is better suited to transferring large chunks of commands, such as via DMA. 

### "Command word" definition

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-7    | Command Index 0
| 8-15   | Command Index 1
| 16-23  | Command Index 2
| 24-31  | Command Index 3

#### Notes:
When packing multiple commands you may not leave zeroed indicies (indicating no command) in between non-zeroed indicies. Meaning that when sending one command the top 24 bits must be zero, when sending 2 commands the top 16 bits must be zero, and when sending 3 commands the top 8 bits must be zero.

## Command Summary

| Command Index | Port address | Summary                              |
|---------------|--------------|--------------------------------------|
| 0x10          | 0x0400_0440  | <a href=3d_matrix_commands.html#3D_CMD_10>Select Matrix Mode/Matrix Stack</a>
| 0x11          | 0x0400_0444  | <a href=3d_matrix_commands.html#3D_CMD_11>Push matrix onto stack</a>
| 0x12          | 0x0400_0448  | <a href=3d_matrix_commands.html#3D_CMD_12>Pull matrix from stack</a>
| 0x13          | 0x0400_044C  | <a href=3d_matrix_commands.html#3D_CMD_13>Store matrix on stack</a>
| 0x14          | 0x0400_0450  | <a href=3d_matrix_commands.html#3D_CMD_14>Restore matrix from stack</a>
| 0x15          | 0x0400_0454  | <a href=3d_matrix_commands.html#3D_CMD_15>Set current matrix to ``I`` (Unit/Identity matrix)</a>
| 0x16          | 0x0400_0458  | <a href=3d_matrix_commands.html#3D_CMD_16>Load 4x4 values into current matrix.</a>
| 0x17          | 0x0400_045C  | <a href=3d_matrix_commands.html#3D_CMD_17>Load 4x3 values into current matrix.</a>
| 0x18          | 0x0400_0460  | <a href=3d_matrix_commands.html#3D_CMD_18>Multiply current matrix by 4x4 Matrix</a>
| 0x19          | 0x0400_0464  | <a href=3d_matrix_commands.html#3D_CMD_19>Multiply current matrix by 4x3 Matrix</a>
| 0x1A          | 0x0400_0468  | <a href=3d_matrix_commands.html#3D_CMD_1A>Multiply current matrix by 3x3 Matrix</a>
| 0x1B          | 0x0400_046C  | <a href=3d_matrix_commands.html#3D_CMD_1B>Scale current matrix by vector</a>
| 0x1C          | 0x0400_0470  | <a href=3d_matrix_commands.html#3D_CMD_1C>Translate current matrix by vector</a>
| 0x20          | 0x0400_0480  | <a href=3d_vertex_polygon_commands.html#3D_CMD_20>Set Vertex Color</a>
| 0x21          | 0x0400_0484  | <a href=3d_vertex_polygon_commands.html#3D_CMD_21>Set Vertex Normal</a>
| 0x22          | 0x0400_0488  | <a href=3d_vertex_polygon_commands.html#3D_CMD_22>Set Vertex Texture Coordinates</a>
| 0x23          | 0x0400_048C  | <a href=3d_vertex_polygon_commands.html#3D_CMD_23>Set Vertex Position (16-bit)</a>
| 0x24          | 0x0400_0490  | <a href=3d_vertex_polygon_commands.html#3D_CMD_24>Set Vertex Position (10-bit)</a>
| 0x25          | 0x0400_0494  | <a href=3d_vertex_polygon_commands.html#3D_CMD_25>Set Vertex XY position</a>
| 0x26          | 0x0400_0498  | <a href=3d_vertex_polygon_commands.html#3D_CMD_26>Set Vertex XZ position</a>
| 0x27          | 0x0400_049C  | <a href=3d_vertex_polygon_commands.html#3D_CMD_27>Set Vertex YZ position</a>
| 0x28          | 0x0400_04A0  | <a href=3d_vertex_polygon_commands.html#3D_CMD_28>Set Vertex position relative to the last</a>
| 0x29          | 0x0400_04A4  | <a href=3d_vertex_polygon_commands.html#3D_CMD_29>Set Vertex Attributes</a>
| 0x2A          | 0x0400_04A8  | <a href=3d_vertex_polygon_commands.html#3D_CMD_2A>Set Texture Parameters</a>
| 0x2B          | 0x0400_04AC  | <a href=3d_vertex_polygon_commands.html#3D_CMD_2B>Set Texture Palette</a>
| 0x30          | 0x0400_04C0  | <a href=3d_vertex_polygon_commands.html#3D_CMD_30>Set Diffused/Ambient Color</a>
| 0x31          | 0x0400_04C4  | <a href=3d_vertex_polygon_commands.html#3D_CMD_31>Set Specular/Emissive Color</a>
| 0x32          | 0x0400_04C8  | Set Light Direction 
| 0x33          | 0x0400_04CC  | Set Light Color
| 0x34          | 0x0400_04D0  | Set Shininess table
| 0x40          | 0x0400_0500  | <a href=3d_vertex_polygon_commands.html#3D_CMD_40>Start vertex list</a>
| 0x41          | 0x0400_0504  | <a href=3d_vertex_polygon_commands.html#3D_CMD_41>End vertex list</a>
| 0x50          | 0x0400_0540  | <a href=3d_vital_commands.html#3D_CMD_50>Swap Buffers between geometry and rendering engine</a>
| 0x60          | 0x0400_0580  | <a href=3d_vital_commands.html#3D_CMD_60>Set 3D Viewport</a>
| 0x70          | 0x0400_05C0  | Test view volume against cuboid
| 0x71          | 0x0400_05C4  | Set Position Vector for test
| 0x72          | 0x0400_05C8  | Set Direction Vector for test

