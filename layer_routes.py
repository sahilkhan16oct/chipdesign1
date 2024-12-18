import os
import json
from dotenv import load_dotenv
from flask import Blueprint, jsonify, request , send_file
from flask_jwt_extended import jwt_required, get_jwt_identity
import zipfile
from io import BytesIO
from subsDecorator import subscription_required  # Assuming your subscription decorator is in subsDecorator.py
from pymongo import MongoClient  # Ensure you have this imported if using MongoDB


load_dotenv()

# Initialize MongoDB client and select collection
mongo_uri = f"mongodb+srv://innoveotech:{os.getenv('DB_PASSWORD')}@azeem.af86m.mongodb.net/chipdesign1?retryWrites=true&w=majority"
client = MongoClient(mongo_uri)
db = client['chipdesign1']  # Replace with your actual database name
layermap_collection = db['layermap']  # Collection to store layermap files

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

# Helper function to get layermap_url based on username
def get_layermap_url(username):
    layermap_entry = layermap_collection.find_one({"username": username})
    print(layermap_entry)
    return layermap_entry["layermap_url"] if layermap_entry else None

# Define the routes
@layer_bp.route('/user/layers', methods=['GET'])
@jwt_required()
# @subscription_required("icurate")
def get_user_layers():
    username = get_jwt_identity() 
    print(username) # Get the username from JWT
    layermap_url = get_layermap_url(username)  # Fetch layermap URL from DB

    if not layermap_url:
        return jsonify({"message": "Layermap not found for this user"}), 404

    layers = load_user_layers(layermap_url)
    return jsonify(layers)

@layer_bp.route('/user/layers', methods=['POST'])
@jwt_required()
# @subscription_required("icurate")
def save_user_all_layers():
    username = get_jwt_identity()  # Get the username from JWT
    layermap_url = get_layermap_url(username)  # Fetch layermap URL from DB

    if not layermap_url:
        return jsonify({"message": "Layermap not found for this user"}), 404

    data = request.json
    save_user_layers(layermap_url, data)
    return jsonify({"message": "Layers saved successfully."})

@layer_bp.route('/user/layers/update', methods=['PUT'])
@jwt_required()
# @subscription_required("icurate")
def update_user_layer():
    username = get_jwt_identity()  # Get the username from JWT
    layermap_url = get_layermap_url(username)  # Fetch layermap URL from DB

    if not layermap_url:
        return jsonify({"message": "Layermap not found for this user"}), 404

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
# @subscription_required("icurate")
def delete_user_layer():
    username = get_jwt_identity()  # Get the username from JWT
    layermap_url = get_layermap_url(username)  # Fetch layermap URL from DB

    if not layermap_url:
        return jsonify({"message": "Layermap not found for this user"}), 404

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
    
@layer_bp.route("/download-all", methods=["GET"])
@jwt_required()
def download_all_files():

    BASE_DIR = os.path.dirname(os.path.abspath(__file__))  
    SAMPLE_DIR = os.path.join(BASE_DIR, "sample")

    # Check if the directory exists
    if not os.path.exists(SAMPLE_DIR):
        return jsonify({"message": "Files directory does not exist"}), 404

    memory_file = BytesIO()

    # Add files to a ZIP file
    with zipfile.ZipFile(memory_file, "w", zipfile.ZIP_DEFLATED) as zf:
        for root, _, files in os.walk(SAMPLE_DIR):
            for file in files:
                file_path = os.path.join(root, file)
                zf.write(file_path, arcname=file)  # Add to ZIP

    memory_file.seek(0)

    # Send the ZIP file
    return send_file(
        memory_file,
        mimetype="application/zip",
        as_attachment=True,
        download_name="sample_files.zip",
    )


@layer_bp.route('/prelimlef', methods=['GET'])
@jwt_required()
# @subscription_required("prelimlef")
def prelimlef():
    return jsonify({"message": "This is the route for prelimlef"}), 200
