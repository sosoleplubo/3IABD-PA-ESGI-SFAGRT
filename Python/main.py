import PAESGISFAGRT  # 👈 important : R majuscule

def oracle(x):
    return 3*x**4 + 48*x**3 - 864*x**2

res = PAESGISFAGRT.recuitsimule(
    oracle,
    0.0001,
    1.0,
    0.99
)

print(res)