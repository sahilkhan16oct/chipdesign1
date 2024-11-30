
# Validation Rules

This document describes the rules for validating GDS files.

## Table of Contents

1. [Rule Types](#rule-types)
2. [Action Types](#action-types)
3. [Detailed Rules](#detailed-rules)
4. [Supported Types](#supported-types)

## Rule Types <a name="rule-types"></a>

- **`angle`** [Angle](#angle)
- **`check_shape`** [CheckShape](#check_shape)
- **`connectivity`** [Connectivity](#connectivity)
- **`density`** [Density](#density)
- **`exact_size`** [ExactSize](#exact_size)
- **`extension`** [Extension](#extension)
- **`filter`** [Filter](#filter)
- **`lvs`** [Lvs](#lvs)
- **`min_area`** [MinArea](#min_area)
- **`min_max_width`** [MinMaxWidth](#min_max_width)
- **`notch`** [Notch](#notch)
- **`off_grid`** [OffGrid](#off_grid)
- **`poly_enclosure`** [PolyEnclosure](#poly_enclosure)
- **`rect_enclosure`** [RectEnclosure](#rect_enclosure)
- **`slit_length`** [SlitLength](#slit_length)
- **`spacing`** [Spacing](#spacing)

## Action Types <a name="action-types"></a>

- **`slice`** [Slice](#slice)

## Detailed Rules <a name="detailed-rules"></a>

### Angle <a name="angle"></a>

- `viewport`: [Viewport](#viewport)
- `is_45_allowed`: *Option(bool)* `// false if not present`

### CheckShape <a name="check_shape"></a>

- `viewport`: [Viewport](#viewport)
- `form`: [ShapeForm](#shape_form)

### Connectivity <a name="connectivity"></a>

- `labels`: `Array(Array([i32, i32]))`
- `layer_stack`: `Array(Array([i32, i32]))`
- `filters`: *Option(Array([Viewport](#viewport)))*
- `only_errors`: *Option(bool)* `// true if not present`

### Density <a name="density"></a>

- `viewport`: [Viewport](#viewport)
- `density`: `float`
- `sampling_level`: `int`
- `window_width`: `float`
- `window_height`: `float`
- `result_type`: [DensityResultType](#density_result_type)
- `is_debug`: *Option(bool)* `// false if not present`

### ExactSize  <a name="exact_size"></a>

- `viewport`: [Viewport](#viewport)
- `width`: `float`
- `height`: `float`

### Extension  <a name="extension"></a>

- `poly`: [Viewport](#viewport)
- `diffusion`: [Viewport](#viewport)
- `value`: `float`

### Filter  <a name="filter"></a>

- `viewport`: [Viewport](#viewport)

### Lvs  <a name="lvs"></a>

- `labels`: `Array(Array([i32, i32]))`
- `layer_stack`: `Array(Array([i32, i32]))`
- `poly`: [Viewport](#viewport)
- `diffusion`: [Viewport](#viewport)
- `transistors`: [Transistor](#transistor)

### PolyEnclosure <a name="poly_enclosure"></a>

- `outer`: [Viewport](#viewport)
- `inner`: [Viewport](#viewport)
- `value`: `float`
- `direction`: [SpaceDirection](#space_direction)

### RectEnclosure  <a name="rect_enclosure"></a>

- `outer`: [Viewport](#viewport)
- `rects`: [Viewport](#viewport)
- `expression`: `String`
- `value_0`: `float`
- `value_1`: *Option(float)* `// null if not present`
- `only_inner`: *Option(bool)* `// true if not present`

### MinArea  <a name="min_area"></a>

- `viewport`: [Viewport](#viewport)
- `min_area`: `float`

### MinMaxWidth  <a name="min_max_width"></a>

- `viewport`: [Viewport](#viewport)
- `algorithm`: [MinMaxAlgorithm](#min_max_algorithm)
- `value`: `float`

### Notch  <a name="notch"></a>

- `viewport`: [Viewport](#viewport)
- `min_length`: `float`

### OffGrid  <a name="off_grid"></a>

- `viewport`: [Viewport](#viewport)
- `step`: `float`

### SlitLength  <a name="slit_length"></a>

- `viewport`: [Viewport](#viewport)
- `algorithm`: [MinMaxAlgorithm](#min_max_algorithm)
- `value`: `float`

### Spacing  <a name="spacing"></a>

- `viewport_0`: [Viewport](#viewport)
- `viewport_1`: [Viewport](#viewport)
- `spacing`: `float`
- `direction`: [SpaceDirection](#space_direction)

## Actions <a name="actions"></a>

### Slice  <a name="slice"></a>

- `cell`: `string`
- `layer_data`: [Layer](#layer)
- `orientation`: [Orientation](#orientation)
- `slice_type`: [SliceType](#slice_type)
- `self_only`: `bool`

## Supported Types <a name="supported-types"></a>

### DensityResultType <a name="density_result_type"></a>

- **`shapes`**
- **`rects`**
- **`slices`**

### SpaceDirection <a name="space_direction"></a>

- **`x`**
- **`y`**
- **`xy`**
- **`dxy`**

### MinMaxAlgorithm <a name="min_max_algorithm"></a>

- **`min_width_x`**
- **`min_width_y`**
- **`min_width_xy`**
- **`min_width_dxy`**
- **`max_width_x`**
- **`max_width_y`**
- **`max_width_xy`**

### Viewport <a name="viewport"></a>

- **`base`**: [Layer](#layer)
- **`filter`**: *Option([LayerFilter](#layer_filter))*

### Layer <a name="layer"></a>

- **`layer_number`**: `int`
- **`datatype_number`**: `int`

### LayerFilter <a name="layer_filter"></a>

- **`areas`**: `Array([Area](#area))`

### Area <a name="area"></a>

- **`layer`**: [Layer](#layer)
- **`operation`**: [AreaOperation](#area_operation)

### AreaOperation <a name="area_operation"></a>

- **`union`**
- **`intersection`**
- **`difference`**
- **`exclusion`**
- **`select`**
- **`invert_select`**
- **`select_full`**
- **`invert_select_full`**

### Orientation <a name="orientation"></a>

- **`vertical`**
- **`horizontal`**

### SliceType <a name="slice_type"></a>

- **`origin`**
- **`outer_box`**

### ShapeForm <a name="shape_form"></a>

- **`square`**
- **`vertical`**
- **`horizontal`**

### Transistor <a name="transistor"></a>

- **`name`**: `string`
- **`id_prefix`**: `string`
- **`power_supply`**: `string`
- **`source_drain_implant`**: [Layer](#layer)
- **`body_implant`**: [Layer](#layer)
- **`x_well`**: *Option([Layer](#layer))* `// null if not present`
