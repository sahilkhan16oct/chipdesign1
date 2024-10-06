import json
import os
import sys
from io import FileIO
import gdstk


def convert_gds_to_json(file_path: str) -> dict:
    """Reads a GDS file from a given path and outputs a JSON structure with the layout data."""
    gds = gdstk.read_gds(file_path)

    cells_json = []
    covered_cells = set()
    for top_cell in gds.top_level():
        cells_json = cell_to_json(top_cell, covered_cells, cells_json)

    cells_json.reverse()
    output_data = {
        "layout_data": {
            "units": {
                "user_size": gds.precision / gds.unit,
                "db_size": gds.precision,
            },
            "cells": cells_json,
        }
    }

    return output_data


def convert_path_to_polygons(cell: gdstk.Cell):
    for path in cell.paths:
        polygons = path.to_polygons()
        for polygon in polygons:
            cell.add(polygon)


def cell_to_json(cell: gdstk.Cell, covered_cells: set, cells_json: list) -> dict:
    """Converts a cell and its contents to a JSON-ready dictionary."""
    convert_path_to_polygons(cell)

    properties = []

    for poly in cell.polygons:
        properties.append(
            {
                "type": "Polygon",
                "layer_number": poly.layer,
                "datatype_number": poly.datatype,
                "coordinates": poly.points.tolist(),
            }
        )

    for label in cell.labels:
        properties.append(
            {
                "type": "Label",
                "layer_number": label.layer,
                "datatype_number": label.texttype,
                "coordinates": [label.origin],
                "name": label.text,
                "rotation": label.rotation,
            }
        )

    for inst in cell.references:
        properties.append(
            {
                "type": "Instance",
                "origin": [inst.origin],
                "bounding_box": list(inst.bounding_box()),
                "name": inst.cell.name,
                "rotation": inst.rotation,
                "mirror_x": inst.x_reflection,
            }
        )

    # Recursively handle dependencies
    for dep in cell.dependencies(False):
        if dep.name not in covered_cells:
            covered_cells.add(dep.name)
            cell_to_json(dep, covered_cells, cells_json)

    cells_json.append({"cell_name": cell.name, "properties": properties})
    return cells_json


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py <inputfile.gds>")
        sys.exit(1)

    input_file = sys.argv[1]
    filename = os.path.basename(input_file).strip(".gds")

    output_data = convert_gds_to_json(input_file)
    with open(f"{filename}.json", "w") as json_file:
        json.dump(output_data, json_file, indent=4)
