from flask_jwt_extended import get_jwt
from functools import wraps
from datetime import datetime
from flask import jsonify

def subscription_required(subscription_type):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            claims = get_jwt()
            
            # Access subscription details directly
            subscription = claims.get(subscription_type, {})
            print(f"Subscription details for {subscription_type}: {subscription}")

            # Check if subscription is active
            if not subscription.get("active", False):
                return jsonify({"message": f"Access denied. {subscription_type} subscription is required."}), 403

            # Parse start and end dates
            start_date_str = subscription.get("startDate")
            end_date_str = subscription.get("endDate")
            current_date = datetime.now()

            # Check date range if start and end dates are provided
            if start_date_str and end_date_str:
                try:
                    # Updated date parsing to match the JWT format
                    start_date = datetime.strptime(start_date_str, '%a, %d %b %Y %H:%M:%S %Z')
                    end_date = datetime.strptime(end_date_str, '%a, %d %b %Y %H:%M:%S %Z')
                except ValueError:
                    return jsonify({"message": "Invalid date format in subscription."}), 400

                # Validate if current date is within the subscription period
                if not (start_date <= current_date <= end_date):
                    return jsonify({"message": f"Your {subscription_type} subscription has expired."}), 403

            return func(*args, **kwargs)
        return wrapper
    return decorator
