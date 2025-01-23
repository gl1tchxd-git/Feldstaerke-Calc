import open3d as o3d
import os
import copy
import numpy as np
import pandas as pd

np.random.seed(42)

o3d.__version__
# Open3D version used in this exercise: 0.16.0

#Axis
mesh_coord_frame = o3d.geometry.TriangleMesh.create_coordinate_frame(size=5, origin=[0, 0, 0])

mesh_conductor = o3d.geometry.TriangleMesh.create_cylinder(radius=0.1, height=0.5)
mesh_conductor.compute_vertex_normals()

o3d.visualization.Visualizer.get_view_control()
o3d.visualization.draw_geometries([mesh_conductor, mesh_coord_frame])