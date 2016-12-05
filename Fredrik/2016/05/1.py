from hashlib import md5
import re
import sys

door_ID = sys.stdin.readline().strip('\n')

password = ''
index = 0

while True:
    string_to_hash = door_ID + str(index)
    hash_object = md5(string_to_hash.encode('utf-8'))
    hash_str = hash_object.hexdigest()
    if re.match(r'^00000', hash_str):
        password += hash_str[5:6]
        print(index)

    index += 1

    if len(password) == 8:
        break

print(password)
    
