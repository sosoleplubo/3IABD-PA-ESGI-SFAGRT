import requests
import os
import csv
import random
from dotenv import load_dotenv

load_dotenv()
API_KEY = os.getenv("TMDB_API_KEY")

movies = []

# récupérer une page aléatoire
page = random.randint(1, 50)

discover = requests.get(
    "https://api.themoviedb.org/3/discover/movie",
    params={"api_key": API_KEY, "page": page}
).json()

for film in discover["results"][:10]:

    movie_id = film["id"]

    details = requests.get(
        f"https://api.themoviedb.org/3/movie/{movie_id}",
        params={"api_key": API_KEY}
    ).json()

    credits = requests.get(
        f"https://api.themoviedb.org/3/movie/{movie_id}/credits",
        params={"api_key": API_KEY}
    ).json()

    director = next((c["name"] for c in credits["crew"] if c["job"] == "Director"), "")
    actors = ", ".join(a["name"] for a in credits["cast"][:3])

    studio = ", ".join(c["name"] for c in details["production_companies"])
    genre = ", ".join(g["name"] for g in details["genres"])
    countries = ", ".join(c["name"] for c in details["production_countries"])

    row = [
        studio,
        genre,
        details["release_date"],
        details["budget"],
        details["runtime"],
        director,
        details["title"],
        actors,
        details["revenue"],
        countries
    ]

    movies.append(row)

with open("Data/v1dataset.csv", "w", newline="", encoding="utf-8") as f:
    writer = csv.writer(f)
    writer.writerow(["Studio","Genre","Date","Budget","Durée","Réalisateur","Noms","Acteurs","BoxOffice","Pays"])
    writer.writerows(movies)

print("10 films ajoutés au dataset")