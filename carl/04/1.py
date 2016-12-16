import collections 

def run():
    with open("input") as f:
        lines = f.readlines()
    
    data = []
    [data.append(line.replace("\r\n", "")) for line in lines]

    sector_id_sum = 0
    for entry in data:
        sector_id = entry.split("-")[-1].split("[")[0]
        checksum = entry.split(sector_id)[1].replace("[", "").replace("]", "")
        encryption = "".join(entry.split("-")[:-1])

        most_common_letters = []
        most_common = sorted(collections.Counter(encryption).most_common(), key=lambda tup: (-tup[1], tup[0]))
        [most_common_letters.append(most_common[i][0]) for i in range(0, 5)]

        valid_room = True
        for char in checksum:
            if char not in most_common_letters:
                valid_room = False

        if valid_room:
            sector_id_sum += int(sector_id)

    print sector_id_sum


run()