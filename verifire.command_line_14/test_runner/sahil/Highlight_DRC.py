import gdstk
import re
import json
import sys

def create_poly(points, lay, dtype):
	return gdstk.Polygon(points, layer = lay, datatype = dtype)

def create_line(points, lay, dtype, dbu):
	line = gdstk.rectangle(
			corner1 = (points[0][0], points[0][1]),
			corner2 = (points[1][0] + 1 / dbu, points[1][1] + + 1 / dbu),
			layer = lay,
			datatype = dtype
		)

#	path = gdstk.FlexPath(points[0],1/dbu, layer = lay, datatype = dtype)
#	path.segment(points[1])
	return line
		
def generate_layer(rule_name, layer_data):
	rule = rule_name.split()[0]
	if rule == "MinMaxWidth":
		rtype = "W"
	elif rule == "Spacing":
		rtype = "S"
	elif rule == "MinArea":
		rtype = "A"
	elif rule == "RectEnclosure":
		rtype = "Enc"
	elif rule == "Angle":
		rtype = "Angle"
	elif rule == "Extension":
		rtype = "Ext"
	elif rule == "ExactSize":
		rtype = "size"
	
	matches = re.findall(r'\d+/\d+', rule_name,re.I)

	lay, dtype = matches[0].split("/")
	lay2 = None
	dtype2 = None
	lay2_name = None
	flag = 0
	flag2 = 1
	drc_lay_name = None
	
	if len(matches) > 1:
		lay2, dtype2 = matches[1].split("/")
		flag2 = 0
	
	for lay_info in layer_data['layers']:
		if lay_info["layer_number"] == lay and lay_info["datatype_number"] == dtype:
			lay_name = lay_info["layer_name"]
			flag = 1
				
		if lay2 != None and dtype2 != None:
			if lay_info["layer_number"] == lay2 and lay_info["datatype_number"] == dtype2:
				lay2_name = lay_info["layer_name"]
				flag2 = 1
		
		if flag == 1 and flag2 == 1:
			if lay2_name != None:
				drc_lay_name = lay_name+"."+lay2_name+"."+rtype
			else:
				drc_lay_name = lay_name+"."+rtype
					
		if lay_info["layer_name"] == drc_lay_name:
			drc_lay_num = int(lay_info["layer_number"])
			drc_dtype_num = int(lay_info["datatype_number"])
			break
		
	return drc_lay_num,drc_dtype_num
		
		
def read_file(gds_file, rve_file):
	with open("default_layermap.json",'r') as layermap:
		layer_data = json.load(layermap)
		
	lib = gdstk.read_gds(gds_file)
	
	f = open(rve_file,'r')
	f = f.readlines()
	
	cell = lib[f[0].split()[0]]
	dbu = int(f[0].split()[1])
	
	line = 1
	while line < len(f):
		rule_name = f[line]
		#print("rule name", rule_name)
		violation_cnt = int(f[line+1].split()[0])
		#print("violation_cnt", violation_cnt)
		line += 2
		
		while violation_cnt != 0:
			p_cnt = int(f[line].split()[-1])
			p_type = f[line].split()[0]
			point_list = []
			
			for i in range(line+1, line+p_cnt+1):
				point = f[i].replace("\n",'')
				point = point.split()
				if p_type == 'p':
					point_list.append((int(point[0])/dbu, int(point[1])/dbu))
				elif p_type == 'e':
					point_list.append((int(point[0])/dbu, int(point[1])/dbu))
					point_list.append((int(point[2])/dbu, int(point[3])/dbu))
			
			lay, dtype = generate_layer(rule_name, layer_data)
			#print(lay, dtype)
			if p_type == 'p':
				poly = create_poly(point_list, lay, dtype)
			elif p_type == 'e':
				poly = create_line(point_list, lay, dtype, dbu)
			
			print(poly)	
			cell.add(poly)
			
			line += p_cnt+1	
			violation_cnt -= 1	
				
	lib.write_gds(f"{username}_DRC_GDS.gds")
	
if __name__ == "__main__":
    # Get the username passed as an argument
    username = sys.argv[1]  # Assuming the username is passed as the first argument
    read_file(f"{username}_cells.gds", f"./{username}_cells.rve")
		
