import pandas as pd
import numpy as np
import PAESGISFAGRT
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler

import shutil
import os

for d in ["../logs/train", "../logs/test"]:
    if os.path.exists(d):
        shutil.rmtree(d)

df = pd.read_csv("../../Data/old/v1datasetclear.csv")

taille = 1000
df = df.sample(n=taille, random_state=19)

y = df["BoxOffice"].values.reshape(-1, 1)
X = df.drop(columns=["BoxOffice"])

X = pd.get_dummies(X)

X = X.values.astype(np.float64)
y = y.astype(np.float64)

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

scaler_X = StandardScaler()
scaler_y = StandardScaler()

X_train = scaler_X.fit_transform(X_train)
X_test  = scaler_X.transform(X_test)

y_train = scaler_y.fit_transform(y_train)
y_test  = scaler_y.transform(y_test)

dataset_train = [(X_train[i].tolist(), y_train[i].tolist()) for i in range(len(X_train))]
dataset_test  = [(X_test[i].tolist(),  y_test[i].tolist())  for i in range(len(X_test))]

nb_features = X_train.shape[1]

model = PAESGISFAGRT.ModelePMC(
    couches=[nb_features, 16, 16, 1],
    txapprentissage=0.01,
    activation="tanh",
    probleme="regression",
    logdir_train="../logs/train",
    logdir_test="../logs/test",
)

model.entrainer(dataset_train, dataset_test, nbgeneration=1000)
model.sauvegarder("../../Modele/PMC2.json")

prediction = []
verite = []

for x, y_val in dataset_train:
    prediction.append(model.predire(x)[0])
    verite.append(y_val[0])

prediction = np.array(prediction)
verite = np.array(verite)

ecart = np.mean((verite - prediction) ** 2)
print("Ecart train (normalisé) :", ecart)

prediction_test = []
verite_test = []

for x, y_val in dataset_test:
    prediction_test.append(model.predire(x)[0])
    verite_test.append(y_val[0])

prediction_test = np.array(prediction_test)
verite_test = np.array(verite_test)

ecart_test = np.mean((verite_test - prediction_test) ** 2)
print("Ecart test  (normalisé) :", ecart_test)

# Ecart en valeurs réelles (dénormalisé)
pred_reel = scaler_y.inverse_transform(prediction_test.reshape(-1, 1))
vrai_reel = scaler_y.inverse_transform(verite_test.reshape(-1, 1))
ecart_reel = np.mean((vrai_reel - pred_reel) ** 2)
print("Ecart test  (réel $)    :", ecart_reel)
print("RMSE test   (réel $)    :", np.sqrt(ecart_reel))