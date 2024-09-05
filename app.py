from flask import Flask, jsonify, request, session
from flask_cors import CORS  # Import CORS from flask_cors
import json
import os
from jsonToGds import convert_json_to_gds 
from gdsToJson import convert_gds_to_json

app = Flask(__name__)
CORS(app)  # Enable CORS for your frontend origin

app.secret_key = 'your_secret_key'  # Needed for session management

# Dummy user data (usually fetched from a database)
users = {
        "admin1": "12345",
        "admin2": "123456789"
    }

@app.route('/login', methods=['POST'])
def login():
        data = request.json
        username = data.get('username')
        password = data.get('password')

        if username in users and users[username] == password:
            session['user'] = username
            return jsonify({"message": "Login successful", "authenticated": True})
        else:
            return jsonify({"message": "Invalid credentials", "authenticated": False}), 401

@app.route('/logout', methods=['POST'])
def logout():
        session.pop('user', None)
        return jsonify({"message": "Logged out", "authenticated": False})

@app.route('/check-auth', methods=['GET'])
def check_auth():
    if 'user' in session:
        return jsonify({"authenticated": True})
    return jsonify({"authenticated": False}), 401




# Correct path to the layer map file
LAYERS_FILE_PATH = '/home/chipdesign1/chipdesign1/layermap.json'

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
@app.route('/convert-and-save-gds', methods=['POST'])
def convert_to_gds():
        data = request.json
        json_content = data.get('json_content', '')
        project_name = data.get('project_name', '')
        print(json_content)
        if not json_content or not project_name:
            return jsonify({"message": "JSON content or project name is missing"}), 400

        # Define the path for the GDS file
        output_dir = os.path.join(os.getcwd(), "gds_output")
        os.makedirs(output_dir, exist_ok=True)
        gds_filename = os.path.join(output_dir, f"{project_name}.gds")

        try:
            # Save the JSON content temporarily to a file
            temp_json_path = os.path.join(output_dir, f"{project_name}.json")
            with open(temp_json_path, 'w') as temp_json_file:
                temp_json_file.write(json_content)

            # Convert the JSON to GDS using your existing conversion logic
            convert_json_to_gds(temp_json_path, gds_filename)

            return jsonify({"message": "Conversion successful", "gds_file": gds_filename}), 200

        except Exception as e:
            return jsonify({"message": f"Conversion failed: {str(e)}"}), 500

@app.route('/convert-gds-to-json', methods=['POST'])
def convert_gds_to_json_route():
        try:
            # Get the JSON data from the request
            data = request.json
            file_name = data.get('file_name')
            directory_path = data.get('directory_path')

            if not file_name or not directory_path:
                return jsonify({'message': 'file_name or directory_path is missing'}), 400

            # Ensure the directory path is absolute
            directory_path = os.path.abspath(directory_path)

            # Build the full file path
            file_path = os.path.join(directory_path, file_name)

            if not os.path.exists(file_path):
                return jsonify({'message': 'File does not exist'}), 404

            # Open the GDS file and convert it to JSON
            json_data = convert_gds_to_json(file_path)  # Passing file path, not the file object

            return jsonify({'json_data': json_data}), 200
        
        except Exception as e:
            return jsonify({'message': str(e)}), 500

