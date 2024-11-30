# Step 1: Check Python version
python --version

# Step 2: Create and activate virtual environment
python -m venv venv
source venv/bin/activate  # On Windows, use .\venv\Scripts\activate

# Step 3: Install requirements
pip install -r requirements.txt

# Step 4: Run application with Gunicorn
gunicorn -w 4 -b 0.0.0.0:8000 app:app
