import os
import json
from flask import Blueprint, jsonify, request
from flask_jwt_extended import jwt_required, get_jwt
from subsDecorator import subscription_required  # Assuming your subscription decorator is in subsDecorator.py

# Create a Blueprint
layer_bp = Blueprint('layer_bp', __name__)

# Function to load layers specific to the user
def load_user_layers(layermap_url):
    if layermap_url and os.path.exists(layermap_url):
        with open(layermap_url, 'r') as file:
            return json.load(file)
    else:
        return {"layers": []}

# Function to save layers specific to the user
def save_user_layers(layermap_url, data):
    if layermap_url:
        with open(layermap_url, 'w') as file:
            json.dump(data, file, indent=4)

# Define the routes
@layer_bp.route('/user/layers', methods=['GET'])
@jwt_required()
@subscription_required("icurate")
def get_user_layers():
    claims = get_jwt()
    layermap_url = claims.get("layermap_url")
    # print(claims)
    layers = load_user_layers(layermap_url)
    return jsonify(layers)

@layer_bp.route('/user/layers', methods=['POST'])
@jwt_required()
@subscription_required("icurate")
def save_user_all_layers():
    claims = get_jwt()
    layermap_url = claims.get("layermap_url")

    data = request.json
    save_user_layers(layermap_url, data)
    return jsonify({"message": "Layers saved successfully."})

@layer_bp.route('/user/layers/update', methods=['PUT'])
@jwt_required()
@subscription_required("icurate")
def update_user_layer():
    claims = get_jwt()
    layermap_url = claims.get("layermap_url")

    data = request.json
    layers = load_user_layers(layermap_url)

    for i, layer in enumerate(layers["layers"]):
        if layer["layer_number"] == data["layer_number"] and layer["datatype_number"] == data["datatype_number"]:
            layers["layers"][i] = data
            save_user_layers(layermap_url, layers)
            return jsonify({"message": "Layer updated successfully."})

    return jsonify({"message": "Layer not found."}), 404

@layer_bp.route('/user/layers/delete', methods=['DELETE'])
@jwt_required()
@subscription_required("icurate")
def delete_user_layer():
    claims = get_jwt()
    layermap_url = claims.get("layermap_url")

    data = request.json
    layers = load_user_layers(layermap_url)

    new_layers = [
        layer for layer in layers["layers"]
        if not (layer["layer_number"] == data["layer_number"] and layer["datatype_number"] == data["datatype_number"])
    ]

    if len(new_layers) != len(layers["layers"]):
        save_user_layers(layermap_url, {"layers": new_layers})
        return jsonify({"message": "Layer deleted successfully."})
    else:
        return jsonify({"message": "Layer not found."}), 404
    



@layer_bp.route('/prelimlef',methods=['GET'])
@jwt_required()
@subscription_required("prelimlef")
def prelimlef():
    return jsonify({"message": "this is route of prelimlef "}), 200