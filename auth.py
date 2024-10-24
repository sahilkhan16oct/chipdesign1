import time
from flask import Blueprint, jsonify, request, session
from pymongo import MongoClient
from werkzeug.security import generate_password_hash, check_password_hash
import os
from dotenv import load_dotenv
import random
import smtplib
from email.mime.text import MIMEText

load_dotenv()

# Initialize MongoDB client
mongo_uri = f"mongodb+srv://innoveotech:{os.getenv('DB_PASSWORD')}@azeem.af86m.mongodb.net/chipdesign1?retryWrites=true&w=majority"
client = MongoClient(mongo_uri)
db = client['chipdesign1']
users_collection = db['users']

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

@auth_bp.route('/signup', methods=['POST'])
def signup():
    data = request.json
    username = data.get('username')
    email = data.get('email')
    occupation = data.get('occupation')
    password = data.get('password')

    # Check if required fields are present
    if not all([username, email, occupation, password]):
        return jsonify({"message": "All fields (username, email, occupation, password) are required"}), 400

    # Check if user already exists
    if users_collection.find_one({"username": username}) or users_collection.find_one({"email": email}):
        return jsonify({"message": "User with this username or email already exists"}), 400

    # Create new user
    hashed_password = generate_password_hash(password)
    users_collection.insert_one({
        "username": username,
        "email": email,
        "password": hashed_password,
        "is_verified": False,
        "occupation": occupation,
        "subscription": {
            "module5": False,
            "module4": False,
            "module3": False
        },
        "startDate": None,
        "endDate": None
    })

    # Generate and send OTP
    otp = generate_otp()
    users_collection.update_one({"username": username}, {"$set": {"otp": otp, "otp_timestamp": time.time()}})
    send_email(email, otp)

    return jsonify({"message": "User created successfully, OTP sent to email"}), 201

@auth_bp.route('/login', methods=['POST'])
def login():
    data = request.json
    username = data.get('username')
    password = data.get('password')

    user = users_collection.find_one({"username": username})

    if user and check_password_hash(user['password'], password):
        if user.get('is_verified'):
            session['user'] = username
            return jsonify({"message": "Login successful", "authenticated": True}), 200
        else:
            return jsonify({"message": "Account not verified. Please verify your email."}), 403
    else:
        return jsonify({"message": "Invalid credentials", "authenticated": False}), 401

@auth_bp.route('/logout', methods=['POST'])
def logout():
    session.pop('user', None)
    return jsonify({"message": "Logged out", "authenticated": False})

@auth_bp.route('/check-auth', methods=['GET'])
def check_auth():
    if 'user' in session:
        return jsonify({"authenticated": True})
    return jsonify({"authenticated": False}), 401
