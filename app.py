from flask import Flask, jsonify, request, send_file 
from flask_cors import CORS
import json
import os
from jsonToGds import convert_json_to_gds 
from gdsToJson import convert_gds_to_json
import platform
from io import BytesIO
import tempfile
from auth import auth_bp
from layer_routes import layer_bp
from DRc import drc_bp
from flask_jwt_extended import JWTManager
from datetime import timedelta


app = Flask(__name__)
CORS(app)  # Enable CORS for your frontend origin

system = platform.system()

app.secret_key = 'your_secret_key'
app.config["JWT_SECRET_KEY"] = "your_jwt_secret_key" 
app.config["JWT_ACCESS_TOKEN_EXPIRES"] = timedelta(days=1)
jwt = JWTManager(app)  

@app.route('/', methods=['GET'])
def home():
    return jsonify({"message": "Yes, this site is working"})


# authentication url
app.register_blueprint(auth_bp, url_prefix='/auth')

app.register_blueprint(layer_bp,url_prefix='/layers')

app.register_blueprint(drc_bp,url_prefix='/drc')

# Path to the layer map file
LAYERS_FILE_PATH = 'layermap.json'

    # Load layers from the JSON file
def load_layers():
        if os.path.exists(LAYERS_FILE_PATH):
            with open(LAYERS_FILE_PATH, 'r') as file:
                return json.load(file)
        else:
            return {"layers": []}

    # Save layers to the JSON file
def save_layers(data):
        with open(LAYERS_FILE_PATH, 'w') as file:
            json.dump(data, file, indent=4)

    # Endpoint to get all layers
@app.route('/layers', methods=['GET'])
def get_layers():
        layers = load_layers()
        return jsonify(layers)

    # Endpoint to save all layers
@app.route('/layers', methods=['POST'])
def save_all_layers():
        data = request.json
        save_layers(data)
        return jsonify({"message": "Layers saved successfully."})

    # Endpoint to update a specific layer
@app.route('/layers/update', methods=['PUT'])
def update_layer():
        data = request.json
        layers = load_layers()

        for i, layer in enumerate(layers["layers"]):
            if layer["layer_number"] == data["layer_number"] and layer["datatype_number"] == data["datatype_number"]:
                layers["layers"][i] = data
                save_layers(layers)
                return jsonify({"message": "Layer updated successfully."})

        return jsonify({"message": "Layer not found."}), 404

    # Endpoint to delete a specific layer
@app.route('/layers/delete', methods=['DELETE'])
def delete_layer():
        data = request.json
        layers = load_layers()

        new_layers = [layer for layer in layers["layers"] if not (layer["layer_number"] == data["layer_number"] and layer["datatype_number"] == data["datatype_number"])]

        if len(new_layers) != len(layers["layers"]):
            save_layers({"layers": new_layers})
            return jsonify({"message": "Layer deleted successfully."})
        else:
            return jsonify({"message": "Layer not found."}), 404

    # Endpoint to upload and overwrite the layer map
@app.route('/upload-layermap', methods=['POST'])
def upload_layermap():
        if 'file' not in request.files:
            return jsonify({"message": "No file part"}), 400
        
        file = request.files['file']

        if file.filename == '':
            return jsonify({"message": "No selected file"}), 400

        if file:
            file.save(LAYERS_FILE_PATH)
            return jsonify({"message": "File uploaded and layers updated successfully."}), 200

# Define the BASE_DIR
if system == "Windows":
        BASE_DIR = os.path.join(os.path.expanduser("~"), "Downloads")
elif system == "Darwin":  # macOS
        BASE_DIR = os.path.join(os.path.expanduser("~"), "Documents")
else:  # Linux and other systems
        BASE_DIR = os.path.join(os.path.expanduser("~"), "Documents")


@app.route('/convert-and-save-gds', methods=['POST'])
def convert_and_save_gds():
        data = request.json
        json_content = data.get('json_content', '')
        project_name = data.get('project_name', '')
          # This is just the directory name

        if not json_content or not project_name :
            return jsonify({"message": "JSON content, project name, or directory path is missing"}), 400

        # Build the full directory path using BASE_DIR and directory_name
        #full_directory_path = os.path.join(BASE_DIR, directory_name)
        #os.makedirs(BASE_DIR, exist_ok=True)  # Create the directory if it doesn't exist

        # Define the path for the GDS file
        gds_filename = os.path.join(BASE_DIR, f"{project_name}.gds")

        try:
            # Save the JSON content temporarily to a file
            temp_json_path = os.path.join(BASE_DIR, f"{project_name}.json")
            with open(temp_json_path, 'w') as temp_json_file:
                temp_json_file.write(json_content)

            # Convert the JSON to GDS using your existing conversion logic
            convert_json_to_gds(temp_json_path, gds_filename)

            # After saving, serve the GDS file for download
            return send_file(gds_filename, as_attachment=True, download_name=f"{project_name}.gds", mimetype="application/octet-stream")

        except Exception as e:
            return jsonify({"message": f"Conversion failed: {str(e)}"}), 500

import os
import tempfile
from flask import request, jsonify

@app.route('/convert-gds-to-json', methods=['POST'])
def convert_gds_to_json_route():
    try:
        # Check if a file was uploaded
        if 'file' not in request.files:
            return jsonify({"message": "No file uploaded"}), 400

        # Get the uploaded GDS file
        file = request.files['file']
        
        # Ensure the file has a valid GDS extension
        if not file.filename.endswith('.gds'):
            return jsonify({"message": "Invalid file type, expected a GDS file"}), 400
        
        # Try reading the file to check if it's valid
        try:
            file_content = file.read()
        except Exception as e:
            return jsonify({"message": f"Error reading input file: {str(e)}"}), 400

        # Check the file size (6 KB limit)
        if len(file_content) > 8 * 1024:  # 8 KB = 8 * 1024 bytes
            return jsonify({"message": "File size exceeds the 8 KB limit"}), 400

        # Save the GDS file to a temporary location
        with tempfile.NamedTemporaryFile(delete=False, suffix=".gds") as temp_gds_file:
            temp_gds_file.write(file_content)
            temp_gds_file_path = temp_gds_file.name

        try:
            # Convert the GDS file to JSON using the file path
            json_data = convert_gds_to_json(temp_gds_file_path)  # Pass the temp file path to the function

            # Return the JSON data as a response
            return jsonify({'json_data': json_data})

        finally:
            # Clean up: delete the temporary GDS file
            if os.path.exists(temp_gds_file_path):
                os.remove(temp_gds_file_path)

    except Exception as e:
        return jsonify({'message': str(e)}), 500




if __name__ == '__main__':
    app.run(host='0.0.0.0',port=8000)