import os
import pandas as pd
import shutil
import secrets
from pymongo import MongoClient
from dotenv import load_dotenv
from werkzeug.security import generate_password_hash

load_dotenv()

# MongoDB connection setup
mongo_uri = f"mongodb+srv://innoveotech:{os.getenv('DB_PASSWORD')}@azeem.af86m.mongodb.net/chipdesign1?retryWrites=true&w=majority"
client = MongoClient(mongo_uri)
db = client['chipdesign1']
users_collection = db['users']
layermap_collection = db["layermap"]

# File and directory setup
LAYERS_DIR = "layermap"
BASE_LAYERS_FILE = "layermap.json"
os.makedirs(LAYERS_DIR, exist_ok=True)  # Ensure the directory exists

def generate_password():
    """Generate an 8-character random password."""
    return secrets.token_urlsafe(6)  # Generates a secure random string

def register_users_one_by_one(file_path):
    """Process each user in the file one by one."""
    # Read the Excel file
    df = pd.read_excel(file_path)
    
    for index, row in df.iterrows():
        # username = row['c_id']
        email = row['email']
        # username = email.split('@')[0]

        # Check if the user already exists
        if users_collection.find_one({"username": email}):
            print(f"User {email} already exists. Skipping...")
            df.at[index, 'status'] = 'Already Exists'
            continue

        # Generate and hash password
        password = generate_password()
        hashed_password = generate_password_hash(password)

        print("ID:", email)
        print("Generated Password:", password)
        print("Hashed Password:", hashed_password)
        
        # Insert user into the database
        user_entry = {
            "username": email,
            "email": email,
            "password": hashed_password,
            "counter": 24
        }
        users_collection.insert_one(user_entry)
        print(f"Inserted user: {email}")
        
        # Create a layermap file for the user
        new_layermap_file = f"{LAYERS_DIR}/{email}_layermap.json"
        shutil.copyfile(BASE_LAYERS_FILE, new_layermap_file)
        
        # Insert the layermap entry into the `layermap` collection
        layermap_entry = {
            "username": email,
            "layermap_url": new_layermap_file,
        }
        layermap_collection.insert_one(layermap_entry)
        print(f"Created layermap for user: {email}")
        
        # Update the current row in the Excel file
        df.at[index, 'username'] = email  
        df.at[index, 'password'] = password  # Save plain password for reference
        df.at[index, 'status'] = 'Inserted'

    # Save the updated DataFrame back to the same file
    output_path = file_path.replace('.xlsx', '_updated.xlsx')
    df.to_excel(output_path, index=False)
    print(f"Registration completed. Updated file saved to: {output_path}")

# File path to the input Excel file
file_path = "details.xlsx"
register_users_one_by_one(file_path)
