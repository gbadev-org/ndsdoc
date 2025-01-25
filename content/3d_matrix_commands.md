# 3D Matrix Commands
On the DS there are multiple matrix operations as well as matrix stacks responsible for calculating the resulting light, normal, and position vectors related to any given vertex. These matrix stacks are:

* Projection stack (Size: 1, Mode: 0)
* Coordinate stack (Size: 32, Mode: 1,2)
* Directional stack (Size: 32, Mode: 1,2)
* Texture stack: (Size: 1, Mode: 3)

Mode in this case reffers to what index needs to be sent to CMD 0x10 in order to "select" the given stack. Notice how the coordinate and directional stack share modes 1 and 2. Which indicates how both are changed when in those modes (and also internally share the same stack pointer). The two modes however change both stacks in different ways depending on the commands used. 

When working with the matrix commands, you may often stumble upon the term "Current matrix", which reffers to the matrix at the top of the selected matrix stack(s).

# Matrix Stack Commands

Here are the commands which depends on the matrix stack(s):

<a id="3D_CMD_10"></a>
## Set Matrix Mode: Port 0x04000440, Index 0x10, 1 Parameter
Selects the matrix stack which is to be modified

Parameter Definition:

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-1    | Mode Index
| 2-31   | Unused

Possible Mode Indicies:

| Mode Index | Selected Stack and purpose |
|------------|----------------------------|
| 0          | Selects Projection Matrix, in a traditional MVP matrix this would be the P part.
| 1          | Position and Direction Matrix stack. In a traditional MVP matrix this would be the MV parts. 
| 2          | Position and Direction Matrix stack. Primarily used in lighting and test related context.
| 3          | Selects Texture Coordinate Matrix stack. Modifies how texture coordinates are transformed.


<a id="3D_CMD_11"></a>
## Push Matrix stack: Port 0x04000444, Index 0x11, No Parameter
Pushes the "Current matrix" onto the stack

<a id="3D_CMD_12"></a>
## Pull Matrix stack: Port 0x04000448, Index 0x12, 1 Parameter
Pops N matricies off the current matrix stack

Parameter Definition:

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-5    | Ammount of matricies to pop in range -30..=31 (N)
| 6-31   | Unused

Notes: 

On matrix stacks with a size of 1, the parameter is ignored and 1 is always used.

<a id="3D_CMD_13"></a>
## Store Current Matrix on Stack: Port 0x0400044C, Index 0x13, 1 Parameter
Stores the current matrix on another part of the stack. Leaves the stack pointer unchanged.

Parameter Definition:

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-4    | Stack Offset
| 5-31   | Unused

Note: 

A Stack Offset of 31 causes the stack error flag in the ``GXSTAT`` register to be set.

On stacks where the size is 1, the parameter is ignored and 0 is always used.

<a id="3D_CMD_14"></a>
## Restore Current Matrix from Stack: Port 0x04000450, Index 0x14, 1 Parameter
Sets the current matrix to the values of another matrix on the stack. Leaves the stack pointer unchanged.

Parameter Definition:

| Bit(s) | Description                                             |
|--------|---------------------------------------------------------|
| 0-4    | Stack Offset
| 5-31   | Unused

Note: 

A Stack Offset of 31 causes the stack error flag in the ``GXSTAT`` register to be set.

On stacks where the size is 1, the parameter is ignored and 0 is always used.

# General Matrix Commands

These are the commands which modify the current matrix:

<a id="3D_CMD_15"></a>
## Load Identity Matrix: Port 0x04000454, Index 0x15, No Parameter
Sets the current matrix to a Unit/Identity matrix. Sometimes denoted as ``I`` or ``1``.

<a id="3D_CMD_16"></a>
## Load 4x4 Matrix: Port 0x04000458, Index 0x16, 16 Parameters
Sets the current matrix to a 4x4 matrix, where each parameter corresponds to a cell in the matrix. Values are loaded row by row. starting at row 0 column 0.

<a id="3D_CMD_17"></a>
## Load 4x3 Matrix: Port 0x0400045C, Index 0x17, 12 Parameters
Sets the current matrix to a 4x3 matrix (padded to a 4x4 matrix which doesn't scale W), where each parameter corresponds to a cell in the matrix. Values are loaded row by row. starting at row 0 column 0.

<a id="3D_CMD_18"></a>
## Multiply 4x4 Matrix: Port 0x04000460, Index 0x18, 16 Parameters
Multiply the current matrix by a 4x4 matrix, where each parameter corresponds to a cell in the matrix. Values are loaded row by row. starting at row 0 column 0.

<a id="3D_CMD_19"></a>
## Multiply 4x3 Matrix: Port 0x04000464, Index 0x19, 12 Parameters
Multiply the current matrix by a 4x3 matrix (padded to a 4x4 matrix which doesn't scale W), where each parameter corresponds to a cell in the matrix. Values are loaded row by row. starting at row 0 column 0.

<a id="3D_CMD_1A"></a>
## Multiply 3x3 Matrix: Port 0x04000468, Index 0x1A, 9 Parameters
Multiply the current matrix by a 3x3 matrix (padded to a 4x4 matrix which makes W unnaffected), where each parameter corresponds to a cell in the matrix. Values are loaded row by row. starting at row 0 column 0.

<a id="3D_CMD_1B"></a>
## Scale Current Matrix by vector: Port 0x0400046C, Index 0x1B, 3 Parameters
Multiply the current matrix by a scale matrix where each scalar value corresponds to the coordinates in a vector. (W coordinate is always 1)

<a id="3D_CMD_1C"></a>
## Translate Current Matrix by vector: Port 0x04000470, Index 0x1C, 3 Parameters
Multiply the current matrix by a translation matrix containing the supplied vector coordinates.



