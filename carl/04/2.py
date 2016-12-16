import collections 

def run():
    with open("input") as f:
        lines = f.readlines()
    
    data = []
    [data.append(line.replace("\r\n", "")) for line in lines]

    alphabet = 'abcdefghijklmnopqrstuvwxyz' 
    for entry in data:
        sector_id = entry.split("-")[-1].split("[")[0]
        checksum = entry.split(sector_id)[1].replace("[", "").replace("]", "")
        encryption = "".join(entry.split(sector_id)[0].replace("-", " ")).lower()

        result_str = ""
        for c in encryption:
            if c in alphabet:
                c = alphabet[(alphabet.index(c) + int(sector_id)) % 26]
                result_str += c

        if "northpoleobjects" in result_str:
            print sector_id


run()