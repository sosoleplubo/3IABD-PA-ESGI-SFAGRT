import requests
import os
from dotenv import load_dotenv

load_dotenv()
API_KEY = os.getenv("TMDB_API_KEY")

movie = "Inception"

# recherche film
r = requests.get(
    "https://api.themoviedb.org/3/search/movie",
    params={"api_key": API_KEY, "query": movie}
).json()

movie_id = r["results"][0]["id"]

# détails
details = requests.get(
    f"https://api.themoviedb.org/3/movie/{movie_id}",
    params={"api_key": API_KEY}
).json()

# crédits
credits = requests.get(
    f"https://api.themoviedb.org/3/movie/{movie_id}/credits",
    params={"api_key": API_KEY}
).json()

director = next((c["name"] for c in credits["crew"] if c["job"] == "Director"), "")
actors = ", ".join(a["name"] for a in credits["cast"][:3])

genre = ", ".join(g["name"] for g in details["genres"])
studio = ", ".join(c["name"] for c in details["production_companies"])
countries = ", ".join(c["name"] for c in details["production_countries"])
franchise = details["belongs_to_collection"]["name"] if details["belongs_to_collection"] else ""

print("Genre,Studio,Date,Budget,BudgetCom,Durée,Réalisateur,Noms,Acteurs,BoxOffice,Pays")

print(
    f'"{genre}",'
    f'"{studio}",'
    f'{details["release_date"]},'
    f'{details["budget"]},'
    f','
    f'{details["runtime"]},'
    f'"{director}",'
    f'"{franchise}",'
    f'"{actors}",'
    f'{details["revenue"]},'
    f'"{countries}"'
)