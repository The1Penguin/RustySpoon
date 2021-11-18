# Python program to read
# json file
 
import json
 
# Opening JSON file
f = open('items.json',)
b = open('nodes.json',)

a = {
   "items": [

   ]
}
 
# returns JSON object as
# a dictionary
data = json.load(f)
data2 = json.load(b)
 
# Iterating through the json
# list
for i in data2['nodes']:
    v = {}
    v["location"] = i["zone"]
    v["start"] = i["startTime"]
    v["end"] = i["endTime"]
    list = i["itemIds"]
    for j in data['items']:
        if any(map(lambda x: x==j["id"], list)):
            v["name"] = j["name"]
    a["items"].append(v)

with open("out.json", "w") as outfile:
    json.dump(a, outfile)
 
# Closing file
f.close()
b.close()
