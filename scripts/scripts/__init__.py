import os
from concurrent import futures
from dataclasses import dataclass
from typing import Iterable

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
        os.makedirs(os.path.dirname(self.path), exist_ok=True)
        with open(self.path, "wb") as f:
            f.write(response.content)
        print(f"Downloaded {self.url} -> {self.path}.")


def download_csv_files_if_missing(files: Iterable[CsvFile]):
    with futures.ThreadPoolExecutor() as executor:
        for _ in executor.map(CsvFile.download_if_missing, files):
            pass
