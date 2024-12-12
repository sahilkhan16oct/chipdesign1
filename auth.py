import time
from datetime import datetime, timedelta
from flask import Blueprint, jsonify, request, session
from pymongo import MongoClient
from werkzeug.security import generate_password_hash, check_password_hash
import os
from dotenv import load_dotenv
import random
import smtplib
from email.mime.text import MIMEText
from functools import wraps
from flask_jwt_extended import create_access_token, jwt_required , get_jwt
import shutil


load_dotenv()

# Initialize MongoDB client
mongo_uri = f"mongodb+srv://innoveotech:{os.getenv('DB_PASSWORD')}@azeem.af86m.mongodb.net/chipdesign1?retryWrites=true&w=majority"
client = MongoClient(mongo_uri)
db = client['chipdesign1']
users_collection = db['users']
layermap_collection = db["layermap"]

auth_bp = Blueprint('auth', __name__)

# Constants
OTP_VALIDITY_DURATION = 300  # OTP valid for 5 minutes (300 seconds)

def generate_otp():
    return str(random.randint(100000, 999999))

def send_email(to_email, otp):
    subject = "Your OTP Code"
    body = f"Your OTP code is {otp}. It is valid for 5 minutes."
    
    msg = MIMEText(body)
    msg['Subject'] = subject
    msg['From'] = os.getenv('EMAIL_ADDRESS')  # Your Gmail address
    msg['To'] = to_email

    with smtplib.SMTP_SSL('smtp.gmail.com', 465) as server:
        server.login(os.getenv('EMAIL_ADDRESS'), os.getenv('EMAIL_PASSWORD'))
        server.sendmail(os.getenv('EMAIL_ADDRESS'), to_email, msg.as_string())

@auth_bp.route('/verify-otp', methods=['POST'])
def verify_otp():
    data = request.json
    username = data.get('username')
    otp = data.get('otp')

    print(f"Received OTP verification request for username: {username} with OTP: {otp}")

    # Fetch the user from the database
    user = users_collection.find_one({"username": username})
    if user and 'otp' in user:
        if user['otp'] == otp:
            # Check if OTP is still valid
            if time.time() - user['otp_timestamp'] <= OTP_VALIDITY_DURATION:
                # Update the user to set is_verified to True
                users_collection.update_one({"username": username}, {"$set": {"is_verified": True}, "$unset": {"otp": "", "otp_timestamp": ""}})
                return jsonify({"message": "OTP verified successfully", "verified": True}), 200
            else:
                return jsonify({"message": "OTP expired", "verified": False}), 400
        else:
            return jsonify({"message": "Invalid OTP", "verified": False}), 400
    return jsonify({"message": "No OTP found for this user", "verified": False}), 400

@auth_bp.route('/resend-otp', methods=['POST'])
def resend_otp():
    data = request.json
    username = data.get('username')

    user = users_collection.find_one({"username": username})
    if user and not user.get('is_verified'):
        otp = generate_otp()
        users_collection.update_one({"username": username}, {"$set": {"otp": otp, "otp_timestamp": time.time()}})
        send_email(user['email'], otp)  # Send OTP to the user's email
        return jsonify({"message": "OTP resent"}), 200
    else:
        return jsonify({"message": "User not found or already verified"}), 400

@auth_bp.route('/forgot-password', methods=['POST'])
def forgot_password():
    data = request.json
    email = data.get('email')

    user = users_collection.find_one({"email": email})
    if user:
        otp = generate_otp()
        users_collection.update_one({"email": email}, {"$set": {"otp": otp, "otp_timestamp": time.time()}})
        send_email(email, otp)  # Send OTP to the user's email
        return jsonify({"message": "OTP sent for password reset"}), 200
    else:
        return jsonify({"message": "Email not found"}), 404

@auth_bp.route('/reset-password', methods=['POST'])
def reset_password():
    data = request.json
    email = data.get('email')
    otp = data.get('otp')
    new_password = data.get('new_password')

    user = users_collection.find_one({"email": email})
    if user and 'otp' in user:
        if user['otp'] == otp:
            if time.time() - user['otp_timestamp'] <= OTP_VALIDITY_DURATION:
                hashed_password = generate_password_hash(new_password)
                users_collection.update_one({"email": email}, {"$set": {"password": hashed_password}, "$unset": {"otp": "", "otp_timestamp": ""}})
                return jsonify({"message": "Password reset successfully"}), 200
            else:
                return jsonify({"message": "OTP expired", "verified": False}), 400
        else:
            return jsonify({"message": "Invalid OTP", "verified": False}), 400
    else:
        return jsonify({"message": "No OTP found for this email", "verified": False}), 400


# Assuming the layermap.json file is located in the root directory
LAYERS_DIR = "layermap"
BASE_LAYERS_FILE = "layermap.json"

# Ensure the layermap directory exists
if not os.path.exists(LAYERS_DIR):
    os.makedirs(LAYERS_DIR)

@auth_bp.route('/signup', methods=['POST'])
def signup():
    data = request.json
    username = data.get('username')
    email = data.get('email')
    password = data.get('password')

    # Check if required fields are present
    if not all([username, email, password]):
        return jsonify({"message": "All fields (username, email, password) are required"}), 400

    # Check if user already exists
    if users_collection.find_one({"username": username}) or users_collection.find_one({"email": email}):
        return jsonify({"message": "User with this username or email already exists"}), 400

    # Create new user
    hashed_password = generate_password_hash(password)
    start_date = datetime.now()
    end_date = start_date + timedelta(days=28)

    

    # Insert user into the database
    users_collection.insert_one({
        "username": username,
        "email": email,
        "password": hashed_password,
        "is_verified": True,
        "counter":20,
        "occupation": "student",
        "subscription": {
            "prelimlef": {
                "active": False,
                "startDate": None,
                "endDate": None
            },
            "icurate": {
                "active": True,
                "startDate": start_date,
                "endDate": end_date
            },
            "mentorme": {
                "active": False,
                "startDate": None,
                "endDate": None
            }
         },

    })

    # Create a separate layermap file for the authenticated user
    new_layermap_file = f"{LAYERS_DIR}/{username}_layermap.json"
    shutil.copyfile(BASE_LAYERS_FILE, new_layermap_file)

    # Insert the layermap entry into the `layermap` collection
    layermap_collection.insert_one({
        "username": username,
        "layermap_url": new_layermap_file,
        
    })

    # Generate and send OTP
    # otp = generate_otp()
    # users_collection.update_one({"username": username}, {"$set": {"otp": otp, "otp_timestamp": time.time()}})
    # send_email(email, otp)

    return jsonify({"message": "User created successfully, OTP sent to email"}), 201

@auth_bp.route('/login', methods=['POST'])
def login():
    data = request.json
    username = data.get('username')
    password = data.get('password')

    user = users_collection.find_one({"username": username})

    if user and check_password_hash(user['password'], password):
        # if not user.get('is_verified'):
        #     return jsonify({"message": "Account not verified. Please verify your email."}), 403

        # # Har subscription ka status aur dates nikaalna
        # subscription = user.get("subscription", {})
       
        # # Simplify JWT claims structure
        # subscriptions_claims = {
        #     "username": username,
        #     "prelimlef": {
        #         "active": subscription.get("prelimlef", {}).get("active", False),
        #         "startDate": subscription.get("prelimlef", {}).get("startDate", ""),
        #         "endDate": subscription.get("prelimlef", {}).get("endDate", "")
        #     },
        #     "icurate": {
        #         "active": subscription.get("icurate", {}).get("active", False),
        #         "startDate": subscription.get("icurate", {}).get("startDate", ""),
        #         "endDate": subscription.get("icurate", {}).get("endDate", "")
        #     },
        #     "mentorme": {
        #         "active": subscription.get("mentorme", {}).get("active", False),
        #         "startDate": subscription.get("mentorme", {}).get("startDate", ""),
        #         "endDate": subscription.get("mentorme", {}).get("endDate", "")
        #     },
            
        # }

        # Create JWT token with additional claims
        # access_token = create_access_token(identity=username, additional_claims=subscriptions_claims)
        access_token = create_access_token(identity=username)
        return jsonify(access_token=access_token, message="Login successful"), 200
    else:
        return jsonify({"message": "Invalid credentials"}), 401


@auth_bp.route('/generate-layermap', methods=['POST'])
@jwt_required()
def generate_layermap():
    # Retrieve authenticated user's username from JWT claims
    jwt_claims = get_jwt()
    username = jwt_claims.get('username')

    if not username:
        return jsonify({"message": "Username not found in JWT claims"}), 401
    
     # Check if a layermap file already exists for this user
    existing_layermap = layermap_collection.find_one({"username": username})
    if existing_layermap:
        return jsonify({"message": "Layermap already exists for this user", "layermap_url": existing_layermap["layermap_url"]}), 200

    # Create a separate layermap file for the authenticated user
    new_layermap_file = f"{LAYERS_DIR}/{username}_layermap.json"
    shutil.copyfile(BASE_LAYERS_FILE, new_layermap_file)

    # Insert the layermap entry into the `layermap` collection
    layermap_collection.insert_one({
        "username": username,
        "layermap_url": new_layermap_file,
        
    })

    return jsonify({"message": "Layermap generated successfully"}), 201
        





@auth_bp.route('/redirect', methods=['POST'])
def auth_user():
    data = request.get_json()
    email = data.get('email')

    if not email:
        return jsonify({"error": "Email is required"}), 400

    # Check if the user already exists in the database
    user = users_collection.find_one({"username": email})

    if user:
        # If user exists, return a JWT token
        username = user['username']
        access_token = create_access_token(identity=username)
        return jsonify(access_token=access_token, message="Login successful"), 200
    
    # If user does not exist, register them
    username = email  # Use email as the username
    new_user = {
        "username": email,
        "email": email,
        "password": None,  # No password initially
        "counter": 24
    }
    users_collection.insert_one(new_user)

    # Create a separate layermap file for the user
    new_layermap_file = f"{LAYERS_DIR}/{username}_layermap.json"
    shutil.copyfile(BASE_LAYERS_FILE, new_layermap_file)

    # Insert the layermap entry into the `layermap` collection
    layermap_collection.insert_one({
        "username": username,
        "layermap_url": new_layermap_file,
    })

    # Generate a JWT token for the new user
    access_token = create_access_token(identity=username)
    return jsonify(access_token=access_token, message="Registration successful"), 201





