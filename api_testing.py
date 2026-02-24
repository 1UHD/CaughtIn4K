import requests
import os
import json
from dotenv import load_dotenv

load_dotenv()

name = "iuhd"
#uuid = "860d353d-1f1e-4356-a059-fec025a2b590"
uuid = "27da5a80-8185-462b-a771-df63f0989c23"
api_key = os.environ["APIKEY"]

path = os.path.join(os.path.expanduser("~"), "Downloads", "output.json")

url = f"https://api.hypixel.net/player?key={api_key}&uuid={uuid}"
#url = f"https://api.mojang.com/users/profiles/minecraft/{name}"
req = requests.get(url)
print(req.status_code)
data = req.json()

with open(path, "w") as f:
    json.dump(data, f, indent=4, ensure_ascii=False, sort_keys=True)
