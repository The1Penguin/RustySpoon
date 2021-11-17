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
for i in range(len(data['items'])):
    v = {}
    v["name"] = data['items'][i]["name"]
    v["start"] = data2['nodes'][i]["startTime"]
    v["end"] = data2['nodes'][i]["endTime"]
    a["items"].append(v)

with open("out.json", "w") as outfile:
    json.dump(a, outfile)
 
# Closing file
f.close()
b.close()
