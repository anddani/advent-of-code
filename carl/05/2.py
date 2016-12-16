from hashlib import md5
import sys

def run():
    _input = "ugkcyxxp"
    password = [None, None, None, None, None, None, None, None] 
    index = 0

    while None in password:
        door_id = _input + str(index)
        md5_hash = md5(door_id.encode('utf-8'))
        md5_hex = md5_hash.hexdigest()

        if md5_hex[:5] == "00000":
            position = md5_hex[5]
            if position.isdigit() and int(position) >= 0 and int(position) < 8 and password[int(position)] is None:
                password[int(position)] = str(md5_hex[6])
        index += 1

    print password

run()