#!/usr/bin/env python3
"""Used to generate the code for the Greek letter enum in symbols.rs."""

GREEK_LOWER = int("03B0", 16)
GREEK_UPPER = int("0390", 16)
# [] indicates unused letters, just so we know to skip a Unicode code point when looping
LETTERS = """
alpha
beta
gamma
delta
epsilon
zeta
eta
theta
iota
kappa
lambda
mu
nu
xi
omicron
pi
rho
[final_sigma]
sigma
tau
upsilon
phi
chi
psi
omega
""".strip().split('\n')

lines = []
for i, letter in enumerate(LETTERS):
    if letter.startswith('['):
        continue

    lower = chr(GREEK_LOWER + i + 1)
    upper = chr(GREEK_UPPER + i + 1)
    lines.append(f'[strum(props(Lower = "{lower}", Upper = "{upper}"))]')
    lines.append(f'{letter.capitalize()},')

print(*[' ' * 4 + line for line in lines], sep='\n')
