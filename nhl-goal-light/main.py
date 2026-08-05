import os
from time import sleep

import requests

URL = "https://api-web.nhle.com/v1/gamecenter/2025030125/boxscore"

API_TOKEN = os.environ.get("API_TOKEN")

response = requests.get(URL, timeout=10)
response.raise_for_status()

game_data = response.json()

#url = "https://api.lifx.com/v1/lights"

headers = {
    "accept": "application/json",
    "Authorization": "Bearer " + API_TOKEN
}

#response = requests.get(url, headers=headers)

#print(response.text)

url2 = "https://api.lifx.com/v1/lights/d073d5859d7c/state"

red = "red saturation:0.5"
blue = "blue"
color = red

score = 2

while True:
    response = requests.get(URL, timeout=10)
    response.raise_for_status()
    print(game_data['awayTeam']['score'])
    sleep(1)

    if score != game_data['awayTeam']['score']:
        score = game_data['awayTeam']['score']
        for x in range(10):
            if x % 2 == 0:
                color = red
            else:        
                color = blue

            payload = {
                "power": "on",
                "color": color,
                "brightness": 1.0,
                "duration": 0.2,
                "fast": False,
            }
            headers = {
                "accept": "application/json",
                "content-type": "application/json",
                "Authorization": "Bearer " + API_TOKEN,
            }

            response = requests.put(url2, json=payload, headers=headers)
            sleep(0.4)

#print(response.text)


# print("Away Team Score:", game_data['awayTeam']['score'])