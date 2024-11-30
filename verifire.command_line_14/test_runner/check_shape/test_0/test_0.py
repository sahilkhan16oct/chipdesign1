import gdspy
import math

lib = gdspy.GdsLibrary('cell_test')

cell = lib.new_cell('MAIN')
poly_0 = gdspy.Polygon([(0, 0), (0, 5), (5, 5), (5, 0)])
poly_1 = gdspy.Polygon([(10, 0), (10, 5), (20, 5), (20, 0)])
poly_2 = gdspy.Polygon([(25, 0), (25, 10), (30, 10), (30, 0)])
cell.add(poly_0)
cell.add(poly_1)
cell.add(poly_2)


lib.write_gds('test_0.gds')
