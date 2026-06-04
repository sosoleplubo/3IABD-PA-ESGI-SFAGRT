import requests
import os
import csv
from dotenv import load_dotenv

load_dotenv()
API_KEY = os.getenv("TMDB_API_KEY")

movies = []

TOTAL_PAGES = 50

for page in range(1, TOTAL_PAGES + 1):

    print(f"Page {page}...")

    discover = requests.get(
        "https://api.themoviedb.org/3/discover/movie",
        params={"api_key": API_KEY, "page": page}
    ).json()

    if "results" not in discover:
        print("Erreur API :", discover)
        continue

    for film in discover["results"]:

        movie_id = film["id"]

        details = requests.get(
            f"https://api.themoviedb.org/3/movie/{movie_id}",
            params={"api_key": API_KEY}
        ).json()

        credits = requests.get(
            f"https://api.themoviedb.org/3/movie/{movie_id}/credits",
            params={"api_key": API_KEY}
        ).json()

        budget = details.get("budget", 0)
        revenue = details.get("revenue", 0)
        runtime = details.get("runtime", 0)

        
        if not budget or not revenue or not runtime:
            continue

        director = next(
            (c["name"] for c in credits.get("crew", []) if c["job"] == "Director"),
            ""
        )

        actors = ", ".join(a["name"] for a in credits.get("cast", [])[:3])

        studio = ", ".join(c["name"] for c in details.get("production_companies", []))
        genre = ", ".join(g["name"] for g in details.get("genres", []))
        countries = ", ".join(c["name"] for c in details.get("production_countries", []))

        row = [
            studio,
            genre,
            details.get("release_date", ""),
            budget,
            runtime,
            director,
            details.get("title", ""),
            actors,
            revenue,
            countries
        ]

        movies.append(row)

with open("../old/v1dataset.csv", "w", newline="", encoding="utf-8") as f:
    writer = csv.writer(f)
    writer.writerow([
        "Studio","Genre","Date","Budget","Durée",
        "Réalisateur","Noms","Acteurs","BoxOffice","Pays"
    ])
    writer.writerows(movies)

print(f"{len(movies)} films ajoutés au dataset")