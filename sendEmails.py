import pandas as pd
import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from dotenv import load_dotenv
import os

load_dotenv()

# SMTP email server configuration
SMTP_SERVER = "smtp.gmail.com"
SMTP_PORT = 587
EMAIL_ADDRESS = os.getenv("EMAIL_ADDRESS")  # Your email address
EMAIL_PASSWORD = os.getenv("EMAIL_PASSWORD")  # Your email app password

def send_email(to_email, subject, body):
    """Send an email to the specified recipient."""
    try:
        # Setup the email headers
        msg = MIMEMultipart()
        msg['From'] = EMAIL_ADDRESS
        msg['To'] = to_email
        msg['Subject'] = subject

        # Attach the email body
        msg.attach(MIMEText(body, 'plain'))

        # Connect to the SMTP server and send the email
        with smtplib.SMTP(SMTP_SERVER, SMTP_PORT) as server:
            server.starttls()  # Upgrade the connection to secure
            server.login(EMAIL_ADDRESS, EMAIL_PASSWORD)
            server.send_message(msg)

        print(f"Email sent to {to_email}")
    except Exception as e:
        print(f"Failed to send email to {to_email}: {e}")

def send_credentials(file_path):
    """Send credentials to users listed in the updated Excel file."""
    # Read the updated Excel file
    df = pd.read_excel(file_path)

    for index, row in df.iterrows():
        username = row['username']
        email = row['email']
        password = row.get('password')  # Get the password if it exists
        status = row.get('status', '')

        email_username = email.split('@')[0]

        # Skip users who were not inserted or already exist
        if status == 'Already Exists' or pd.isna(password):
            print(f"Skipping {username} ({email}) - {status}")
            continue

        # Email content
        subject = "Your Account Credentials"
        body = (
            f"Dear {email_username},\n\n"
            f"Your account has been successfully created. Here are your credentials:\n\n"
            f"Username: {username}\n"
            f"Password: {password}\n\n"
            f"Please keep these credentials safe and secure.\n\n"
            f"Best regards,\n InnoveoTech"
        )

        # Send the email
        send_email(email, subject, body)

# File path to the updated Excel file
file_path = "details_updated.xlsx"
send_credentials(file_path)
