import logging

# Configure the logger
logging.basicConfig(
    level=logging.DEBUG,  # Log everything from DEBUG and above
    format="%(asctime)s [%(levelname)s] %(message)s",  # Log format
    handlers=[
        logging.FileHandler("app.log"),  # Save logs to a file
        logging.StreamHandler()          # Print logs to terminal
    ]
)
logger = logging.getLogger(__name__)
