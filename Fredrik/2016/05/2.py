from hashlib import md5
import re
import sys

door_ID = sys.stdin.readline().strip('\n')

password = {}
valid_positions = [str(x) for x in range(0,8)]
index = 0

while True:
    string_to_hash = door_ID + str(index)
    hash_object = md5(string_to_hash.encode('utf-8'))
    hash_str = hash_object.hexdigest()
    if re.match(r'^00000([0-7])(.)', hash_str):
        password.setdefault(int(hash_str[5:6]),hash_str[6:7]) 
        print(index)
    '''if '00000' == hash_str[:5]:
        if hash_str[5:6] in valid_positions:
            password.setdefault(int(hash_str[5:6]),hash_str[6:7]) 
            print(index)'''

    index += 1

    if len(password) == 8:
        break

print(''.join(password[int(x)] for x in range(0,8)))
    
