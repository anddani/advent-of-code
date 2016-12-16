import re

def has_abba(string):
    for i in range(0, len(string)-3):
        first = string[i+0]
        second = string[i+1]
        third = string[i+2]
        fourth = string[i+3]
        if first != second and (first + second) == (fourth + third):
            return True
    return False


def run():
    with open("input") as f:
        lines = f.readlines()

    TLS_counter = 0
    for line in lines:

        ip = line.replace("\r\n", "")
        hypernets = re.findall(r"\[([A-Za-z0-9_]+)\]", ip)
        ip_strings = re.sub(r"\[([A-Za-z0-9_]+)\]", "-", ip).split("-")

        valid_hypernets = True
        valid_address = False

        for string in hypernets:
            if has_abba(string):
                valid_hypernets = False

        for string in ip_strings:
            if has_abba(string):
                valid_address = True

        if valid_hypernets and valid_address:
            TLS_counter += 1

    print TLS_counter
   
run()