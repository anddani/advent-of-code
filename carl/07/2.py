import re

def get_abas(string):
    abas = []
    for i in range(0, len(string)-2):
        first = string[i+0]
        second = string[i+1]
        third = string[i+2]
        if first == third and first != second:
            abas.append(first + second + third)
    return abas

def has_bab(string, abas):
    for i in range(0, len(string)-2):
        first = string[i+0]
        second = string[i+1]
        third = string[i+2]
        if first == third and first != second and (second + first + second) in abas:
            return True
    return False

def run():
    with open("input") as f:
        lines = f.readlines()

    SSL_counter = 0
    for line in lines:

        ip = line.replace("\r\n", "")
        hypernets = re.findall(r"\[([A-Za-z0-9_]+)\]", ip)
        supernets = re.sub(r"\[([A-Za-z0-9_]+)\]", "-", ip).split("-")

        abas = []
        for supernet in supernets:
            abas += get_abas(supernet)

        for hypernet in hypernets:
            if has_bab(hypernet, abas):
                SSL_counter += 1
                break

    print SSL_counter
   
run()