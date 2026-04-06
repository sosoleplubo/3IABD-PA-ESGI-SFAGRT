import PAESGISFAGRT

print(PAESGISFAGRT.__file__)
print(dir(PAESGISFAGRT))
def oracle(x):
    return (x + 8)**2 * (x + 1)**2 * (x - 6)**2 + 5*x


res_gen = PAESGISFAGRT.genetique(
    oracle,
    100,   # taille_population
    20,    # nb_elites
    500,   # nb_generation
    -100,  # domaine_min
    100    # domaine_max
)

print("Résultat génétique :", res_gen)



res_recuit = PAESGISFAGRT.recuitsimule(
    oracle,
    0.0001,
    1.0,
    0.99
)

print("Résultat recuit :", res_recuit)

dataset = [
    ([0.0, 0.0], 0.0),
    ([0.0, 1.0], 1.0),
    ([1.0, 0.0], 1.0),
    ([1.0, 1.0], 0.0),
]

w1, b1, w2, b2, preds = PAESGISFAGRT.pmc(dataset, 100000, 0.1)

print("w1 =", w1)
print("b1 =", b1)
print("w2 =", w2)
print("b2 =", b2)
print("predictions =", preds)