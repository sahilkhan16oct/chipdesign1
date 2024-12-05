from fpdf import FPDF, XPos, YPos
import re
import json
import sys

def create_pdf():
	pdf = FPDF()
	pdf.set_auto_page_break(auto=True, margin=15)
	pdf.add_page()
	pdf.set_font("Arial", size=12)
	#pdf.set_margins(left, top, right)
	pdf.set_margins(20, 20, 20)
	
	return pdf
	
#--------------------------------------------------------

def get_value(rule):
	val = rule.split()[-1]
	val = val.replace("\n",'')
	
	return val

#--------------------------------------------------------	

def generate_rule(rule_name):
	if rule_name == "MinMaxWidth":
		rtype = "Minimum Width"
	elif rule_name == "Spacing":
		rtype = "Minimum Spacing"
	elif rule_name == "MinArea":
		rtype = "Minimum Area"
	elif rule_name == "RectEnclosure":
		rtype = "Minimum Enclosure"
	elif rule_name == "Angle":
		rtype = "Angle not allowed"
	elif rule_name == "Extension":
		rtype = "Minimum Extension"
	elif rule_name == "ExactSize":
		rtype = "Exact Size"	
		
	return rtype
#--------------------------------------------------------
		
def generate_layer(rule, layer_data):
	matches = re.findall(r'\d+/\d+', rule,re.I)

	lay, dtype = matches[0].split("/")
	lay2 = None
	dtype2 = None
	lay2_name = None
	dtype2_name = None
	
	if len(matches) > 1:
		lay2, dtype2 = matches[1].split("/")
	
	for lay_info in layer_data['layers']:
		if lay_info["layer_number"] == lay and lay_info["datatype_number"] == dtype:
			lay_name = lay_info["layer_name"]
			dtype_name = lay_info["datatype_name"]
				
		if lay2 != None and dtype2 != None:
			if lay_info["layer_number"] == lay2 and lay_info["datatype_number"] == dtype2:
				lay2_name = lay_info["layer_name"]
				dtype2_name = lay_info["datatype_name"]
		
	return [lay, dtype, lay_name, dtype_name, lay2, dtype2, lay2_name, dtype2_name]
	
#--------------------------------------------------------

def read_file(rve_file):
	with open("default_layermap.json",'r') as layermap:
		layer_data = json.load(layermap)
		
	f = open(rve_file,'r')
	f = f.readlines()
	
	cellname = f[0].split()[0]
	pdf = create_pdf()
	pdf.cell(0, 10, text="Cell name: "+cellname, new_x=XPos.LMARGIN, new_y=YPos.NEXT)
	
	line = 1
	while line < len(f):
		rule = f[line]
		rule_name = generate_rule(rule.split()[0])
		pdf.ln(10)
		pdf.cell(0, 10, text="Rule name: "+rule_name, new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		
		lay_list = generate_layer(rule, layer_data)
		pdf.cell(0, 10, text="Layer number: "+lay_list[0]+"/"+lay_list[1], new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		pdf.cell(0, 10, text="Layer Name: "+lay_list[2]+" "+lay_list[3], new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		
		if lay_list[4] != None and lay_list[5] != None:
			pdf.cell(0, 10, text="Other Layer number: "+lay_list[4]+"/"+lay_list[5], new_x=XPos.LMARGIN, new_y=YPos.NEXT)
			pdf.cell(0, 10, text="Other Layer Name: "+lay_list[6]+" "+lay_list[7], new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		
		value = get_value(rule)
		pdf.cell(0, 10, text="Value: "+str(value), new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		
		violation_cnt = int(f[line+1].split()[0])
		pdf.cell(0, 10, text="Total violations: "+str(violation_cnt), new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		pdf.cell(0, 10, text="Locations: ",new_x=XPos.LMARGIN, new_y=YPos.NEXT)
		#pdf.set_x(pdf.get_x())
		
		line += 2
		
		while violation_cnt != 0:
			p_cnt = int(f[line].split()[-1])
			p_type = f[line].split()[0]
			point_list = []
			string = '['
			
			for i in range(line+1, line+p_cnt+1):
				point = f[i].replace("\n",'')
				point = point.split()
				if p_type == 'p':
					string =  string+" ("+point[0]+", "+point[1]+") "
				elif p_type == 'e':
					string = string+" ("+point[0]+", "+point[1]+") "+"("+point[2]+", "+point[3]+") "
			
			string = string+"]"
			pdf.cell(0, 10, text=string, new_x=XPos.LMARGIN, new_y=YPos.NEXT)
			
			line += p_cnt+1	
			violation_cnt -= 1	
			
	pdf.output(f"{username}_DRC_violations.pdf")

#--------------------------------------------------------
	
if __name__ == "__main__":
	username = sys.argv[1]
	read_file(f"{username}_cells.rve")
			
			
