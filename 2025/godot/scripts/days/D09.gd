class_name D09 extends RefCounted

func parse(data: String) -> Array[Vector2i]:
	var retval: Array[Vector2i] = []
	for line: String in data.split("\n"):
		var n: PackedStringArray = line.split(",")
		assert(n.size() == 2)
		retval.append(Vector2i(n[0].to_int(), n[1].to_int()))
	return retval

func area(a: Vector2i, b: Vector2i) -> int:
	var w: int = 1 + abs(a.x - b.x)
	var h: int = 1 + abs(a.y - b.y)
	return h * w


func is_polygon_contained(polygon_a: PackedVector2Array, polygon_b: PackedVector2Array) -> bool:
	# 1. Check if all vertices of polygon_b are inside polygon_a
	for vertex_b: Vector2 in polygon_b:
		if not Geometry2D.is_point_in_polygon(vertex_b, polygon_a):
			print("Return false from first check")
			return false # A vertex of B is outside A, so B is not fully contained

	# 2. Check for edge intersections (optional, but recommended for robustness)
	# This step is crucial if polygon_a could have holes or if polygon_b could be complex.
	# The 'intersect_polygons' method can help determine if there are any intersections.
	# If B is fully contained within A, and A does not have holes that B could intersect,
	# then there should be no intersection result other than B itself (or an empty array if B is completely removed by a hole in A).
	var intersection_result: Array[PackedVector2Array] = Geometry2D.intersect_polygons(polygon_a, polygon_b)

	# If the intersection result is empty, it might mean B is completely contained within a hole of A,
	# or that B is entirely outside of A. Since we already checked point containment, an empty result
	# here suggests B might be in a hole.
	if intersection_result.is_empty():
		# This case is tricky. If all points of B are in A, but intersection_result is empty,
		# it likely means B is inside a "hole" of A, which means it's NOT contained in the "solid" part.
		# A more robust check might involve analyzing holes or using a different approach like 'clip_polygons'.
		# For simple convex polygons or polygons without holes, an empty intersection_result after point
		# containment check can also indicate no overlap, which contradicts full containment.
		# For the purpose of "fully contained in the solid area", an empty intersection here implies failure.
		return false

	# If intersection_result contains only one polygon and it matches polygon_b,
	# it implies full containment without edge intersections that would break containment.
	# This check is more reliable for simple cases.
	if intersection_result.size() == 1 and intersection_result[0] == polygon_b:
		return true

	# More complex scenarios might require comparing the area of intersection_result[0] with polygon_b's area,
	# or using 'clip_polygons' and checking the result.
	# For a simple "is B fully inside A (and not in a hole of A)", the above checks are usually sufficient.

	return true # If all vertices are inside and no problematic intersections, assume containment.

func points_to_vector2(p: Array[Vector2i]) -> PackedVector2Array:
	var retval: PackedVector2Array = []
	for point: Vector2i in p:
		retval.append(Vector2(point.x, point.y))
	return retval

func part1(data: String) -> String:
	var retval: int = 0
	var points: Array[Vector2i] = parse(data)
	for a: Vector2i in points:
		for b: Vector2i in points:
			var myarea: int = area(a,b)
			if myarea > retval:
				retval = myarea
	return str(retval)
	
func part2(data: String) -> String:
	var retval: int = 0
	var points: Array[Vector2i] = parse(data)
	var polygon: PackedVector2Array = points_to_vector2(points)
	for i: int in range(points.size()):
		var a: Vector2 = Vector2(points[i].x, points[i].y)
		for j: int in range(i+1, points.size()):
			var b: Vector2i = Vector2(points[j].x, points[j].y)
			var p1: Vector2 = Vector2(a.x, b.y)
			var p2: Vector2 = Vector2(b.x, a.y)
			var rectangle: PackedVector2Array = [a, p1, b, p2]
			if is_polygon_contained(rectangle, polygon):
				print("Contained let's go!")
				var myarea: int = area(points[i], points[j])
				if myarea > retval:
					retval = myarea
	return str(retval)
