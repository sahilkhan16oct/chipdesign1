import logging

# Configure the logger
logging.basicConfig(
    level=logging.DEBUG,  # Set the log level to DEBUG for detailed logs
    format="%(asctime)s [%(levelname)s] %(message)s",  # Log format
    handlers=[
        logging.FileHandler("app.log"),  # Logs to a file
        logging.StreamHandler()  # Logs to the console (useful for debugging)
    ]
)
logger = logging.getLogger(__name__)  # Create a logger for this module
