# Vertex and Polygon Commands

This chapter explains the various commands used to send verticies to the 3D hardware in order to construct polygons. On top of this theres also basic lighting, materials, and texture mapping attributes. The DS 3D hardware natively supports constructing both triangles and quadrilaterals (which should not self intersect or have concavities).  

## Usage Overview
Verticies, as well as edges and polygons are to be constructed on the 3D hardware in the following manner:

1. Set Polygon attributes

2. Start a vertex list, where the parameter passed to the start command dictates the type of polygons you wish to construct (Seperated or Connected, Tris or Quads)

3. For each vertex, set relevant vertex attributes in any order. Setting any coordinate of the vertex position sends it to the 3D hardware. Attributes that are left unset will use the values of the previous vertex.

4. End the vertex list.

Notice here that the 3D hardware does not natively support index lists for constructing polygons, 
only a vertex list that constructs either seperated or connected "strips" of primitives. Another 
quirk of the 3D hardware is that the last step is optional, as vertex lists are automatically 
ended when a new one begins or the geometry and rendering engine buffers are swapped. It may 
still be useful however to manually end it for debugging purposes and the sake of "Completeness".






# Vertex List Commands
Commands used to start and end vertex lists, as well as commands which can only be used once per vertex list.

<a id="3D_CMD_40"></a>
## Start Vertex List: Port 0x04000500, Index 0x40, 1 Parameter
Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0      | Primitive Type (0 = triangle, 1 = quadrilateral)
| 1      | Primitive Topology (0 = seperated, 1 = strips)

<a id="3D_CMD_41"></a>
## End Vertex List: Port 0x04000504, Index 0x41, 0 Parameters
Ends a vertex list, as previously discussed this command is optional, and only require for the sake of debugging

<a id="3D_CMD_29"></a>
## Polygon Attributes: Port 0x040004A4, Index 0x29, 1 Parameter
Sets various miscallenous attributes for polygons created by the next vertex list.

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0      | Enable Light 0 
| 1      | Enable Light 1
| 2      | Enable Light 2
| 3      | Enable Light 3
| 4-5    | Polygon Mode (0 = Modulation, 1=Decal, 2=Toon, 3=Shadow)
| 6      | Render Backface
| 7      | Render Frontface
| 8-10   | Unused
| 11     | Set new depth for translucent pixels
| 12     | Render Far-Plane intersecting polygons 
| 13     | Render 1-Dot polygons behind 1-Dot depth
| 14     | Depth test (0 = less than, 1 = equal)
| 15     | Enable Fog
| 16-20  | Alpha (0 = Wireframe, 1..30 = Translucent, 31 = Solid)
| 21-23  | Unused
| 24-29  | Polygon ID 
| 30-31  | Unused 

Notes: If a polygon attribute command is sent while a vertex list is ongoing, it will be deffered until next time a vertex list is begun. (Repeated calls will not stack, only the last written one will be used) Changes in lighting related attributes will not change until a new normal vector is set/calculated.





# Vertex Attribute Commands
These are the commands for various attributes which can be applied to a vertex.

<a id="3D_CMD_20"></a>
## Set Vertex Color: Port 0x04000480, Index 0x20, 1 Parameter
Sets the vertex color used for the next vertex. 

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0-4    | Red
| 5-9    | Green
| 10-14  | Blue
| 15-31  | Unused


<a id="3D_CMD_21"></a>
## Set Normal Vector: Port 0x04000484, Index 0x21, 1 Parameter
Sets the "normal vector" used for the next vertex. In reality this will use the currently set light parameters to calculate a new vertex color.

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0-9    | Vertex X coordinate (sign + 9 bit fraction)
| 10-19  | Vertex Y coordinate (sign + 9 bit fraction)
| 20-29  | Vertex Z coordinate (sign + 9 bit fraction)
| 30-31  | Unused

<a id="3D_CMD_22"></a>
## Set Texture Coordinates: Port 0x04000488, Index 0x22, 1 Parameter
Sets the UV / ST coordinates for the next vertex. 16 units = 1 texel 

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0-15   | S/U coordinate (signed, 4 bit fraction)
| 16-31  | T/V coordinate (signed, 4 bit fraction)

<a id="3D_CMD_2A"></a>
## Set Texture Parameters: Port 0x040004A8, Index 0x2A, 1 Parameter
Binds a texture to the upcoming verticies, along with various texture attributes.

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0-15   | Texture VRAM offset (in steps of 8)
| 16     | Repeat Horizontally (0 = Clamp, 1 = Repeat)
| 17     | Repeat Vertically (0 = Clamp, 1 = Repeat)
| 18     | Flip on horizontal repeat (see notes.)
| 19     | Flip on Vertical repeat (see notes.)
| 20-22  | Horizontal size (see notes.)
| 23-25  | Vertical size (see notes.)
| 26-28  | Texture Format (see notes.)
| 29     | Set color 0 to transparent
| 30-31  | Texture Coordinate transformation mode (see notes.)

Notes:

Texture size is calculated as (8 << N) pixels where N is the specified size. Meaning possible resulting values are 8,16,32,64,128,256,512,1024.

Texture flipping on repeats requires that repeats are on.

When a Texture is clamped, the outmost edge pixels are stretched across any out of bounds pixels.

The various Texture Formats are as follows:

| Index | Description |
|-------|-------------|
| 0     | No texture
| 1     | A315 Translucent Texture
| 2     | 4-Color Paletted texture
| 3     | 16-Color Paletted texture
| 4     | 256-Color Paletted texture
| 5     | Compressed Texture
| 6     | A513 Translucent texture;
| 7     | Bitmap Texture (R5G5B5)

Texture Coordinate Transformation Modes:

| Index | Description |
|-------|-------------|
| 0     | No transformation
| 1     | Texture Coordinate Source
| 2     | Normal Source
| 3     | Vertex Source


<a id="3D_CMD_2B"></a>
## Set Texture Color Palette: Port 0x040004AC, Index 0x2B, 1 Parameter
Set the color palette used for the currently binded texture. Ignored by non-palette based textures.

Parameter Definition: 

| Bit(s) | Description |
|--------|-------------|
| 0-12   | Texture Palette VRAM offset (see notes.)
| 13-31  | Ignored

Notes: When using 4-color paletted textures the offset is calculated in steps of 8. In any other cases it's steps of 16.

<a id="3D_CMD_30"></a>
## Diffuse/Ambient Properties: Port 0x040004C0, index 0x30, 1 Parameter
Sets the Diffused/Ambient properties for the next vertex. 

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0-4    | Diffuse Reflection (Red channel)
| 5-9    | Diffuse Reflection (Green channel)
| 10-14  | Diffuse Refletcion (Blue channel)
| 15     | Set Diffuse reflection as vertex color (0 = No, 1 = Yes)
| 16-20  | Ambient Reflection (Red Channel)
| 21-25  | Ambient Reflection (Green channel)
| 26-30  | Ambient Reflection (Blue channel)
| 31     | Unused

<a id="3D_CMD_31"></a>
## Specular/Emissive Properties: Port 0x040004C4, index 0x31, 1 Parameter
Sets the Specular/Emissive properties for the next vertex. 

Parameter definition:

| Bit(s) | Description |
|--------|-------------|
| 0-4    | Specular Reflection (Red channel)
| 5-9    | Specular Reflection (Green channel)
| 10-14  | Specular Refletcion (Blue channel)
| 15     | Use shininess table (0 = No, 1 = Yes, see chapter on other commands for shininess table details)
| 16-20  | Emission (Red Channel)
| 21-25  | Emission (Green channel)
| 26-30  | Emission (Blue channel)
| 31     | Unused





# Vertex Position Commands
All of these commands send a new vertex to the geometry engine once executes. 
Unless stated otherwise, the vertex coordinates will be a 16bit signed integer 
with a 12bit fraction. (i.e Range is +7.99 to -8.00)

<a id="3D_CMD_23"></a>
## Position Set (16-bit): Port 0x0400048C, Index 0x23, 2 Parameters
Set all coordinates of the next vertex at once. 

Parameter Definition:

| Parameter | Bit(s) | Description |
|-----------|--------|-------------|
| 1         | 0-15   | Vertex X coordinate
| 1         | 16-31  | Vertex Y coordinate
| 2         | 0-15   | Vertex Z coordinate
| 2         | 16-31  | Unused

<a id="3D_CMD_24"></a>
## Position Set (10-bit): Port 0x04000490, Index 0x24, 1 Parameter
Set all coordinates of the next vertex at once, but with less accuracy. 

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-9    | Vertex X coordinate (signed, 6 bit fraction)
| 10-19  | Vertex Y coordinate (signed, 6 bit fraction)
| 20-29  | Vertex Z coordinate (signed, 6 bit fraction)
| 30-31  | Unused

<a id="3D_CMD_25"></a>
## Position Set XY: Port 0x04000494, Index 0x25, 1 Parameter
Set X and Y coordinate of the next vertex. 

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-15   | Vertex X coordinate
| 16-31  | Vertex Y coordinate

<a id="3D_CMD_26"></a>
## Position Set XZ: Port 0x04000498, Index 0x26, 1 Parameter
Set X and Z coordinate of the next vertex. 

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-15   | Vertex X coordinate
| 16-31  | Vertex Z coordinate

<a id="3D_CMD_27"></a>
## Position Set YZ: Port 0x0400049C, Index 0x27, 1 Parameter
Set Y and Z coordinate of the next vertex. 

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-15   | Vertex Y coordinate
| 16-31  | Vertex Z coordinate

<a id="3D_CMD_28"></a>
## Position Set Relative: Port 0x040004A0, Index 0x28, 1 Parameter
Set all coordinates of the next vertex at once, but relative to the last vertex.

Parameter Definition:

| Bit(s) | Description |
|--------|-------------|
| 0-9    | Vertex X coordinate (signed, 12 bit fraction, see notes.)
| 10-19  | Vertex Y coordinate (signed, 12 bit fraction, see notes.)
| 20-29  | Vertex Z coordinate (signed, 12 bit fraction, see notes.)
| 30-31  | Unused

Note: As the value is smaller than the fraction, the relative range is very small (+-0.125)







