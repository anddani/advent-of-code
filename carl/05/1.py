from hashlib import md5
import sys

def run():
    _input = "ugkcyxxp"
    password = ""
    index = 0

    while len(password) < 8:
        door_id = _input + str(index)
        md5_hash = md5(door_id.encode('utf-8'))
        md5_hex = md5_hash.hexdigest()

        if md5_hex[:5] == "00000":
            password += str(md5_hex[5])
        index += 1

    print password

run()