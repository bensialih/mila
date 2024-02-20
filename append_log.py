import time
import os
file_location = "./data/test_file.json"
new_data = dict(name='abe', age=21, address='Birmingham, England')
import json

while True:
    os.system(f"echo '{json.dumps(new_data)}' >> {file_location}")
    time.sleep(0.2)

