extends GdUnitTestSuite
# class_name FooTest

func test_foo() -> void:
	var a: int = 5
	var b: int = 10
	var result: int = min(a,b)
	
	assert_int(result).is_equal(a)
	
func test_bar() -> void:
	assert_int(3).is_equal(3)
