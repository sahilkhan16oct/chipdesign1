import gdstk
import json
import numpy as np
import os
import sys

def convert_json_to_gds(json_file_path, output_gds_path):
    with open(json_file_path, 'r') as json_file:
        data = json.load(json_file)

    lib = gdstk.Library()
    cell_list = []
    instance = {}

    for cells in data['layout_data']['cells']:
        cell_ref = gdstk.Cell(cells['cell_name'])
        lib.add(cell_ref)
        # print(cells)

        cell_list.append(cells['cell_name'])
        instance[cell_ref] = []

        for props in cells['properties']:
            if props['type'] == 'Polygon':
                if 'coordinates' in props and len(props['coordinates']) > 0:
                    cell_ref.add(gdstk.Polygon(tuple(map(tuple, props['coordinates'])), layer=int(props['layer_number']), datatype=int(props['datatype_number'])))
                else:
                    print(f"Warning: Rectangle with missing or invalid coordinates found in {cells['cell_name']}.")

            elif props['type'] == 'Text':
                if 'coordinates' in props and len(props['coordinates']) > 0:
                    rotation = props.get('rotation', 0) or 0
                    cell_ref.add(gdstk.Label(props['text'], origin=tuple(props['coordinates'][0]), rotation=rotation * (np.pi / 180), layer=int(props['layer_number']), texttype=int(props['datatype_number'])))
                else:
                    print(f"Warning: Text with missing or invalid coordinates found in {cells['cell_name']}.")

            elif props['type'] == 'Instance':
                if props['name'] in cell_list:
                    if 'origin' in props and len(props['origin']) > 0:
                        rotation = props.get('rotation', 0) or 0
                        cell_ref.add(gdstk.Reference(props['name'], origin=tuple(props['origin'][0]), rotation=rotation * (np.pi / 180), x_reflection=props.get('mirror_x', False)))
                    else:
                        print(f"Warning: Instance with missing or invalid origin found in {cells['cell_name']}.")
                else:
                    instance[cell_ref].append(props)

    for cell_ref, prop_list in instance.items():
        for props in prop_list:
            if 'origin' in props and len(props['origin']) > 0:
                rotation = props.get('rotation', 0) or 0
                cell_ref.add(gdstk.Reference(props['name'], origin=tuple(props['origin'][0]), rotation=rotation * (np.pi / 180), x_reflection=props.get('mirror_x', False)))
            else:
                print(f"Warning: Deferred Instance with missing or invalid origin found.")

    lib.write_gds(output_gds_path)
    print(f"GDS file saved to {output_gds_path}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python jsonToGds.py <input_json_file> <output_gds_file>")
    else:
        convert_json_to_gds(sys.argv[1], sys.argv[2])
