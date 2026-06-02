import PAESGISFAGRT
import numpy as np

X = np.array([
      [1, 1],
      [2, 3],
      [3, 3]
])
Y = np.array([
      1,
      -1,
      -1
])

dataset = list(zip(X.tolist(), [[y] for y in Y.tolist()]))

model = PAESGISFAGRT.ModelePMC(
    couches=[2, 1],
    txapprentissage=0.01,
    activation="tanh",
    probleme="classification",
)

model.entrainer(dataset, nbgeneration=100000)

for x in X.tolist():
    print(model.predire(x))

model.sauvegarder("../Modele/xor.json")