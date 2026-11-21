extends GdUnitTestSuite


func test_default() -> void:
    var result: BigRange = BigRange.new(2,3)

    assert_int(result.begin).is_equal(2)

func test_sort() -> void:
    var arr: Array[BigRange] = []
    
    arr.append(BigRange.new(5,8))
    arr.append(BigRange.new(1,6))
    arr.append(BigRange.new(3,7))
    
    arr.sort_custom(BigRange.custom_compare)
    
    assert_int(arr[0].begin).is_equal(1)
  
func test_merge() -> void:
    var arr: Array[BigRange] = []
    
    arr.append(BigRange.new(5,8))
    arr.append(BigRange.new(1,3))
    arr.append(BigRange.new(7,9))
    arr.append(BigRange.new(10,11))
    
    var merged: Array[BigRange] = BigRange.merge(arr)
    
    assert_int(merged.size()).is_equal(2)
    assert_int(merged[1].begin).is_equal(5) 

func test_has() -> void:
    var br: BigRange = BigRange.new(5,9)
    
    assert_bool(br.in_range(7)).is_true()
    assert_bool(br.in_range(1)).is_false()
