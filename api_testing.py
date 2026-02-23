import requests
import os
import json
from dotenv import load_dotenv

load_dotenv()

uuid = "860d353d-1f1e-4356-a059-fec025a2b590"
api_key = os.environ["APIKEY"]

path = os.path.join(os.path.expanduser("~"), "Downloads", "output.json")

url = f"https://api.hypixel.net/player?key={api_key}&uuid={uuid}"
req = requests.get(url)
print(req.status_code)
data = req.json()

with open(path, "w") as f:
    json.dump(data, f, indent=4, ensure_ascii=False, sort_keys=True)
