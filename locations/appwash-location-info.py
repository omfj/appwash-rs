#!/usr/local/bin/python3
import os
import csv

import requests

from rich.console import Console

c = Console()

USER_AGENT = "appwash-location-checker 1.0.0"

EMAIL = "<YOUR EMAIL>"
PASSWORD = "<YOUR PASSWORD>"


class User:
    def __init__(self, email, password):
        self.email = email
        self.password = password
        self.token = self.get_token()

    def get_token(self):
        headers = {"Content-Type": "application/json", "User-Agent": f"{USER_AGENT}", "language": "en",
                   "platform": "appWash"}
        data = f'{{ "email": "{self.email}", "password": "{self.password}" }}'

        r = requests.post("https://www.involtum-services.com/api-rest/login", headers=headers, data=data)
        r_json = r.json()
        token = r_json["login"]["token"]

        return token

    def get_headers(self):
        return {"User-Agent": f"{USER_AGENT}", "Referer": "https://appwash.com/", "token": f"{self.token}",
                "language": "NO", "platform": "appWash", "DNT": "1"}


def main():
    user = User(EMAIL, PASSWORD)

    cwd = os.getcwd()

    header = ["LOCATION", "NAME"]

    with open(f"{cwd}/appwash-locations.csv", "a", encoding="UTF-8") as f:
        writer = csv.writer(f)

        writer.writerow(header)

        for location_id in range(0, 11911):
            r = requests.get(f"https://www.involtum-services.com/api-rest/locations/split/{location_id}",
                             headers=user.get_headers())
            r_json = r.json()

            try:
                location_name = r_json["data"]["name"]
                data = [location_id, location_name]

                writer.writerow(data)
                c.log(f"Writing: {data}")
            except KeyError:
                c.log(f"Location {location_id} does not exist.")
                pass


if __name__ == "__main__":
    main()
