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