import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from sklearn.preprocessing import LabelEncoder, StandardScaler
import PAESGISFAGRT

df = pd.read_csv("Data/old/v1dataset.csv")

le = LabelEncoder()
for col in ["Studio", "Genre", "Réalisateur", "Noms", "Acteurs", "Pays"]:
    df[col] = le.fit_transform(df[col].astype(str))


df["Date"] = pd.to_datetime(df["Date"], errors="coerce").dt.year.fillna(0).astype(int)

X = df.drop(columns=["BoxOffice"]).values.astype(float)
df["BoxOffice"] = df["BoxOffice"].astype(str).str.replace(r'[^0-9.]', '', regex=True)
df["BoxOffice"] = pd.to_numeric(df["BoxOffice"], errors="coerce").fillna(0)
Y = df["BoxOffice"].values.astype(float)

x = StandardScaler()
y = StandardScaler()

X = x.fit_transform(X)
Y = y.fit_transform(Y.reshape(-1, 1)).flatten()

# Dataset
dataset = list(zip(X.tolist(), [[y] for y in Y.tolist()]))

# Modèle
nb_features = X.shape[1]
model = PAESGISFAGRT.ModelePMC(
    couches=[nb_features, 16, 16, 1],
    txapprentissage=0.01,
    activation="tanh",
    probleme="regression",
)

losses = []
etape1 = 100
etape = 10

for i in range(etape):
    model.entrainer(dataset, nbgeneration=etape1)

    preds = [model.predire(x)[0] for x, _ in dataset]
    vrais = [y[0] for _, y in dataset]
    loss = np.mean([(p - v) ** 2 for p, v in zip(preds, vrais)])
    losses.append(loss)

    if i % 10 == 0:
        print(f"Étape {i*etape1}/{etape*etape1} — Loss: {loss:.4f}")

# Courbe de loss
plt.plot(losses)
plt.title("Loss")
plt.xlabel(f"Blocs de {etape1} générations")
plt.show()
plt.clf()

preds_denorm = x.inverse_transform([[p] for p in preds])
vrais_denorm = y.inverse_transform([[v] for v in vrais])

plt.scatter(vrais_denorm, preds_denorm, alpha=0.5)
plt.plot([min(vrais_denorm), max(vrais_denorm)],
         [min(vrais_denorm), max(vrais_denorm)], color='red', linestyle='--')
plt.title("Prédictions vs Réalité")
plt.xlabel("BoxOffice réel")
plt.ylabel("BoxOffice prédit")
plt.show()

model.sauvegarder("Modele/boxoffice.json")