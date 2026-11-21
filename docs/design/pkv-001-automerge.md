# PKV-001: Automerge-Based File Format for Pixel Art and Fiber Arts

## 1. Abstract
This document defines a new file format based on Automerge for use in pixel art and fiber arts design. The format uses an indexed approach, allowing each "pixel" to contain extended metadata suitable for physical mediums such as knitting and cross-stitch, going beyond simple color data. The foundational building block is a layered 2D canvas structure, similar to Aseprite's "Cel" concept, which tracks changes across time and space.

## 2. Motivation
Traditional pixel art formats are typically limited to simple color values per pixel. However, fiber arts (e.g., knitting, cross-stitch) require complex metadata for each stitch. For example, a single "pixel" in a cross-stitch pattern may encode both a color/floss index and a specific stitch type (like a quarter or half stitch) or even multiple stitches in a single pixel.

As AI art becomes more ubiquitous, a format that supports capturing the full history becomes an essential tool in demonstrating one's process. Likewise, I personally work on my art often on my phone during commutes but also at home on my PC or tablet. I need to be able to pick up seamlessly without worrying about conflicts, even if I didn't fully remember to sync from one device or the other. Automerge satisfies this requirement quite well and offers flexible data structures that can enable various approaches to conflict resolution when they do occur.

## 3. Specification

The Automerge document contains the following root-level structures:

1. Palette - an indexed representation of all the colors and/or stitches used in the document.
2. Cels - the lowest level representing a flat 2D canvas of pixels (pointers to palette entries). It has it's own width, height, x offset, and y offset so that the disruption from resize operations can be limited to when two devices or two users modify the same cell at a time.
3. Layers - a cel plus some medium specific data structures such as opacity and blend mode for pixel art. Some data would apply to all mediums, such as lock and maybe even alpha lock (assuming some common representation of transparency/no stitch) Multiple layers could point to the same Cel, supporting a Linked Cel feature.
4. Layer Groups - a hierarchical grouping with layers being the leaf nodes. Some nodes have application specific data, such as animation frames, which can be represented as a node in the tree with data such as duration attached to it. Multiple nodes could point to the same layer or layer group node, supporting a Linked Layer / Layer Group feature, which could be useful for backgrounds in animations.
5. Log Entries - these could be rich text, audio clips, video clips, screenshots, or something else I haven't imagined for the purpose of documenting the process. These could include a list of all of the log nodes which they refer to to make the analogoue of a git commit history.

### 3.1 Palette
The Palette is an Automerge List of `paletteEntry` objects.
To avoid dealing with nulls, the List must always have at least one item representing the background color and/or transparency for pixel art, and an empty (no stitch) space for fiber arts.

* `palette`: list of **Items:** `paletteEntry` (maps) containing the following fields:
  * `grayscale8`: Integer between 0 and 255 representing value.
  * `color8` (map): Contains `red`, `green`, `blue`, `alpha` values.
  * `knit` (map):
    * `yarnId` (ObjId): References an entry in the Knit Yarns list.
    * `stitch` (integer): Represents a knitting stitch enum. Valid values (based on `StitchType`) are:
      * `0` (`NO_STITCH`)
      * `1` (`KNIT`)
      * `2` (`PURL`)
  * `crossStitch` (map):
    * `listOfStitch`: A list where each entry specifies:
      * `stitchEnum` (integer): Represents a specific cross-stitch (e.g., full, half, quarter).
      * `flossId` (ObjId): References an entry in the Cross Stitch Flosses list.

### 3.2 Knit
Contains metadata specific to knitting.

* `knit`
  * `yarns`: An Automerge List of `yarnEntry` objects.
    * **Items:** `yarnEntry` (map) containing:
      * `name` (string)
      * `weight` (string)
      * `strands` (integer)    
  * `charts`: A map where the key is the chart's `name` (string) and the value is a chart object:
    * **chart object** (map):
      * `chartType` (integer/enum): The type of knitting chart (e.g., single or double-sided).
      * `layerA` (ObjId): References a Frame `ObjId` for the primary side.
      * `layerB` (optional ObjId): References a Frame `ObjId` for the secondary side (used in double-sided charts).

### 3.3 Cross Stitch
Contains metadata specific to cross-stitch.

* `crossStitch`
  * `flosses`: An Automerge List of `flossEntry` objects.
    * **Items:** `flossEntry` (map) representing floss details.

### 3.4 Canvas
Defines the global canvas boundary.

* `width` (integer)
* `height` (integer)
* `topLeft` (map): Contains `x` and `y` coordinates.

### 3.6 Drawing Data Structures
Drawing data is organized into a hierarchy of frames and layers:

* `frames`: An Automerge List of `frame` objects. A frame is used for charts on fiber arts.
  * `frame` (map):
    * `name` (string): The name of the frame.
    * `layers`: An Automerge List of `layer` objects.
      * `layer` (map):
        * `cel`: A Cel `ObjId`.
        * `opacity` (number, between 0 and 1): opacity/transparency applied to the whole layer.
        * `blendMode` (integer, enum): representation of the equation used to apply this layer to the layer below.
        * `mask`: an optional Cel `ObjId` representing an alpha mask 
        * `visibility` (boolean): true if this layer is rendered.

### 3.7 Cels
The actual pixel data content referenced by the layers.

* `cels`: An Automerge List of `cel` objects.
* `cel` (map):
  * `values` (list of ObjId): The actual indexed pixel data. These ObjIds reference entries in the `Palette` list.
  * `width` (integer)
  * `height` (optional, integer): Can be derived from `width` and the length of the `values` list.
  * `topLeft` (map): Contains `x` and `y` coordinates.
  * `locked` (boolean): if true, this cell should not be edited

## 4. Behavior and Rationale

### 4.1 Coordinate Systems and Resizing
Initially, the `topLeft` coordinate for all Cels and the global Canvas starts at `(0,0)`.

When the canvas is resized, individual Cels do not necessarily need to be modified or repositioned. Instead, the global Canvas `topLeft`, `width`, and `height` are updated.

### 4.2 Dynamic Cel Expansion
If a user draws outside the current bounding area of a specific Cel, that Cel dynamically expands its `topLeft`, `width`, and `height` to accommodate the new drawing operations.

This approach minimizes merge conflicts in a collaborative environment. Conflicts are largely reduced to scenarios where multiple users modify the exact same Cel concurrently after a resize but before synchronization occurs.
