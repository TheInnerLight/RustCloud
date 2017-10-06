
# RustCloud
Rust Point Cloud library

Travis: ![Build Status](https://travis-ci.org/TheInnerLight/RustCloud.svg?branch=master)

[![Waffle.io - Columns and their card count](https://badge.waffle.io/TheInnerLight/RustCloud.png?columns=all)](https://waffle.io/TheInnerLight/RustCloud?utm_source=badge)

# Objectives

Create a Rust library for point cloud processing and analysis.

Features

1) Simple geometry
   1) Fit points to planes
   2) Point to plane distance
   3) Projections 3D -> 2D
2) Spatial partitioning: Answer nearest neighbour queries etc.
   1) Octree
   2) kd-tree
3) Triangulation: Generate polygons/meshes/terrain models.
   1) Delaunay triangulation
4) Registration: Align overlapping point clouds
