import requests
import os
import csv
import random
from dotenv import load_dotenv

load_dotenv()
API_KEY = os.getenv("TMDB_API_KEY")

TARGET_MOVIES = 1000
MAX_PAGE = 1000
movies = []
seen_ids = set()

def fetch_page(page):
    url = "https://api.themoviedb.org/3/discover/movie"
    return requests.get(
        url,
        params={
            "api_key": API_KEY,
            "page": page,
            "sort_by": "popularity.desc"
        }
    ).json()

while len(movies) < TARGET_MOVIES:

    page = random.randint(1, MAX_PAGE)
    print(f"Page random: {page}")

    discover = fetch_page(page)

    if "results" not in discover:
        continue

    for film in discover["results"]:

        movie_id = film["id"]

        if movie_id in seen_ids:
            continue

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

        # filtre important
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
        seen_ids.add(movie_id)

        if len(movies) >= TARGET_MOVIES:
            break

# export CSV
with open("../dataset1000.csv", "w", newline="", encoding="utf-8") as f:
    writer = csv.writer(f)
    writer.writerow([
        "Studio","Genre","Date","Budget","Durée",
        "Réalisateur","Titre","Acteurs","BoxOffice","Pays"
    ])
    writer.writerows(movies)

print(f"OK → {len(movies)} films collectés")