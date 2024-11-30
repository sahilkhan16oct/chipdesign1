import gdspy
import math

lib = gdspy.GdsLibrary('cell_test')

# +
cell_0 = lib.new_cell('CELL_0')
poly_0 = gdspy.Polygon([(-5, -5), (-15, -5), (-15, 5), (-5, 5), (-5, 15), (5, 15), (5, 5), (15, 5), (15, -5), (5, -5), (5, -15), (-5, -15)])
cell_0.add(poly_0)

# H
cell_1 = lib.new_cell('CELL_1')
poly_1 = gdspy.Polygon([(-15, -15), (-15, 15), (-5, 15), (-5, 5), (5, 5), (5, 15), (15, 15), (15, -15), (5, -15), (5, -5), (-5, -5), (-5, -15)])
cell_1.add(poly_1)

# T
cell_2 = lib.new_cell('CELL_2')
poly_2 = gdspy.Polygon([(-15,-15), (-15, -5), (-5, -5), (-5, 5), (-15, 5), (-15, 15), (15, 15), (15, 5), (5, 5), (5, -5), (15, -5), (15,-15)])
cell_2.add(poly_2)

# O
cell_3 = lib.new_cell('CELL_3')
poly_3 = gdspy.Polygon([(-15, -15), (-15, 15), (15, 15), (15, -15), (-15,-15), (-5, -5), (5, -5), (5, 5), (-5, 5), (-5, -5)])
cell_3.add(poly_3)

# O
cell_4 = lib.new_cell('CELL_4')
poly_4 = gdspy.Polygon([(-15, -5), (-15, 5), (-5, 15), (5, 15), (15, 5), (15, -5), (5, -15), (-5, -15)])
cell_4.add(poly_4)

# O
cell_5 = lib.new_cell('CELL_5')
poly_5 = gdspy.Polygon([(-15, -5), (-15, 5), (-5, 15), (5, 15), (15, 5), (15, -5), (5, -15), (-5, -15), (-15, -5), (-5, -5), (5, -5), (5, 5), (-5, 5), (-5, -5)])
cell_5.add(poly_5)



ref_0 = gdspy.CellReference(cell_0, (25, 25), magnification=1.0)
ref_1 = gdspy.CellReference(cell_1, (25, 75), magnification=1.0)

ref_2 = gdspy.CellReference(cell_2, (75, 25), magnification=1.0)
ref_3 = gdspy.CellReference(cell_3, (75, 75), magnification=1.0)

ref_4 = gdspy.CellReference(cell_4, (125, 25), magnification=1.0)
ref_5 = gdspy.CellReference(cell_5, (125, 75), magnification=1.0)

main = lib.new_cell("MAIN")
main.add([ref_0, ref_1, ref_2, ref_3, ref_4, ref_5])

lib.write_gds('test_0.gds')
