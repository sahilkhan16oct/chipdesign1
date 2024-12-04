from flask_jwt_extended import get_jwt_identity, jwt_required
from pymongo import MongoClient
from bson import ObjectId
from flask import jsonify, request, Blueprint ,send_file
import json
import subprocess
import tempfile
from jsonToGds import convert_json_to_gds
import os


# Initialize MongoDB client
mongo_uri = f"mongodb+srv://innoveotech:{os.getenv('DB_PASSWORD')}@azeem.af86m.mongodb.net/chipdesign1?retryWrites=true&w=majority"
client = MongoClient(mongo_uri)
db = client['chipdesign1']
users_collection = db['users']

drc_bp = Blueprint('drc', __name__)

# Maximum allowed size for incoming POST data (4 KB)
MAX_JSON_SIZE = 4 * 1024  # 4 KB

@drc_bp.route('/generate_gds', methods=['POST'])
@jwt_required()
def generate_gds():
    try:
        # Check if the incoming JSON data size is greater than the allowed size
        if request.content_length > MAX_JSON_SIZE:
            return jsonify({"error": "JSON data exceeds the 4 KB size limit"}), 400

        print("limit: ", request.content_length)
        # Parse the incoming JSON data
        data = request.json
        if not data:
            return jsonify({"error": "No JSON data provided"}), 400

        # Retrieve the user's identity (username) from the JWT token
        username = get_jwt_identity()

        # Find the user in the database by their username
        user = users_collection.find_one({"username": username})
        if not user:
            return jsonify({"error": "User not found"}), 404
        
        # Check if the user has any remaining counter
        if user['counter'] <= 0:
            return jsonify({"error": "Counter is at 0. Cannot generate GDS."}), 400

        # Decrease the counter by 1
        users_collection.update_one(
            {"username": username}, 
            {"$inc": {"counter": -1}}  # Decrement counter by 1
        )

        # Get the base directory of the app (root of your repo)
        app_base_dir = os.path.abspath(os.path.dirname(__file__))  # Get the absolute path of the current script

        # Define relative paths from the root of the repo
        test_file_path = os.path.join(app_base_dir, 'verifire.command_line_14', 'test_runner', 'sahil', 'DRC_deck.json')
        output_gds_path = os.path.join(app_base_dir, 'verifire.command_line_14', 'test_runner', 'sahil', f'{username}_cells.gds')  # Use username for GDS filename
        run_sh_path = os.path.join(app_base_dir, 'verifire.command_line_14', 'test_runner', 'sahil', 'run.sh')
        rve_output_path = os.path.join(app_base_dir, 'verifire.command_line_14', 'test_runner', 'sahil', f'{username}_cells.rve')  # RVE file path

        
        # Read and process the test_0.json file
        with open(test_file_path, 'r') as test_file:
            test_data = json.load(test_file)

        # Replace the "cell" key with the value from the incoming request
        test_data['cell'] = data['layout_data']['cells'][0]['cell_name']  # Replace with 'sahil' from the request data
        
        # Replace "input" and "output" keys with the username in DRC_deck.json
        test_data['input'] = f"{username}_cells.gds"
        test_data['output'] = f"{username}_cells.rve"

        # Save the updated test_0.json
        with open(test_file_path, 'w') as test_file:
            json.dump(test_data, test_file, indent=4)
        
        
        # Save JSON data to a temporary file
        temp_json = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
        json.dump(data, temp_json)
        temp_json.close()

        # Convert JSON to GDS
        convert_json_to_gds(temp_json.name, output_gds_path)
        

        # Run the shell script (run.sh)
        
        result = subprocess.run(
            ['bash', run_sh_path], 
            stdout=subprocess.PIPE, 
            stderr=subprocess.PIPE, 
            cwd=os.path.dirname(run_sh_path) 
        )
        

        # Now, run Highlight_DRC.py with the username
        highlight_drc_script_path = os.path.join(app_base_dir, 'verifire.command_line_14', 'test_runner', 'sahil', 'Highlight_DRC.py')
        result = subprocess.run(
            ['python3', highlight_drc_script_path, username],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=os.path.dirname(highlight_drc_script_path)
        )
       

        
        gds_output_path = os.path.join(app_base_dir, 'verifire.command_line_14', 'test_runner', 'sahil', f"{username}_DRC_GDS.gds")
        if os.path.exists(gds_output_path):
            # Send the generated GDS file to the user
            response = send_file(gds_output_path, as_attachment=True)
            
            # Cleanup the files after sending
            os.remove(gds_output_path)
            os.remove(output_gds_path)
            os.remove(rve_output_path)
            os.remove(temp_json.name)

            return response
        else:
            return jsonify({"error": "DRC GDS generation failed"}), 500

    except Exception as e:
        return jsonify({"error": str(e)}), 500
