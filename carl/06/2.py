import collections

def run():
    with open("input") as f:
        lines = f.readlines()

    columns = ["", "", "", "", "", "", "", ""]

    for line in lines:
        message = line.replace("\r\n", "")

        for i in range(0, len(message)):
            columns[i] += message[i]

    result = ""
    for string in columns:
        result += collections.Counter(string).most_common()[-1][0]

    print result

run()