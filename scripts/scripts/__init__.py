import os
from dataclasses import dataclass

import requests


@dataclass
class CsvFile:
    path: str
    url: str

    def download_if_missing(self):
        if os.path.exists(self.path):
            return
        response = requests.get(self.url)
        response.raise_for_status()
        with open(self.path, "wb") as f:
            f.write(response.content)
        print(f"Downloaded {self.url} -> {self.path}.")
