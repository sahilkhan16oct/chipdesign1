import gdspy
import math

lib = gdspy.GdsLibrary('cell_test')

# +
cell_0 = lib.new_cell('CELL_0')
rect = gdspy.Rectangle((0, 0), (100, 100))
cell_0.add(rect)

ref_0 = gdspy.CellReference(cell_0, (0, 0), magnification=1.0)

main = lib.new_cell("MAIN")
main.add([ref_0])

lib.write_gds('origin.gds')
