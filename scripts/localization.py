import json
import re
import sys
import xml.etree.ElementTree as ET

file = sys.argv[1]

item_re = re.compile("@ITEMS_(.*)")

def add_item(db, name, desc):
    match = item_re.match(name)
    if match:
        db[match[1]] = desc

nd = {}
tree = ET.parse(file)
root = tree.getroot()

for child in root[0]:
    tuid = child.attrib['tuid']
    for loc in child:
        if loc.attrib['{http://www.w3.org/XML/1998/namespace}lang'] == 'EN-US':
            for desc in loc:
                add_item(nd, tuid, desc.text)

print(json.dumps(nd))