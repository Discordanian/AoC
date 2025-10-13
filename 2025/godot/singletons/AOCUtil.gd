extends Node
class_name AOCUtil

const NORTH: Vector2i = Vector2i(0, 1)
const SOUTH: Vector2i = Vector2i(0, -1)
const EAST: Vector2i  = Vector2i(1, 0)
const WEST: Vector2i  = Vector2i(-1, 0)

# download input given year day

# given a file reference return Array[String]

# given a file reference return Array[Array[String]]

# Example path given year and day
static func example_path(y: int, d: int) -> String:
    assert(y > 2014, "Year must be > 2014")
    assert(y < 2026, "Year must be < 2026")
    assert(d > - 1, "Day must not be negative")
    assert(d < 26, "Advent of code does not allow days greater than 25")
    return "user://%4d-%02d.ex" % [y,d]

# Input path given year and day
static func input_path(y: int, d: int) -> String:
    assert(y > 2014, "Year must be > 2014")
    assert(y < 2026, "Year must be < 2026")
    assert(d > - 1, "Day must not be negative")
    assert(d < 26, "Advent of code does not allow days greater than 25")
    return "user://%4d-%02d.in" % [y,d]


# Return rows and cols of a 2d array
static func dimensions_of_2d_array(data: Array[Array]) -> Vector2i:
    var x: int = data.size()
    if x > 0:
        return Vector2i(x, data[0].size())
    return Vector2i(0,0)



static func string_to_2d_char_array(input: String) -> Array[Array]:
    var result: Array[Array] = []
    var lines: PackedStringArray = input.split("\n")
    
    for line: String in lines:
        var char_array: Array[String] = []
        for i: int in range(line.length()):
            char_array.append(line[i])
        result.append(char_array)
    
    return result


static func string_to_lines(input: String) -> PackedStringArray:
    return  input.split("\n")
    
static func string_from_file(path: String) -> String:
    var retval: String = ""
    if FileAccess.file_exists(path):
        var file: FileAccess = FileAccess.open(path, FileAccess.READ)
        if file:
           retval = file.get_as_text()
           file.close()
        else:
           push_error("Error reading data from " + path)
    else:
           push_error("Path not found : " + path)
    return retval

static func download_file(parent_node: Node, year: int, day: int, file_path: String) -> void:
    # Get session ID from environment variable
    var session_id: String = OS.get_environment("SESSIONID")
    
    if session_id.is_empty():
        push_error("SESSIONID environment variable not found")
        return
    
    # Create HTTPRequest node
    var http_request: HTTPRequest = HTTPRequest.new()
    parent_node.add_child(http_request)
    
    # Build the URL
    var url: String = "https://adventofcode.com/%d/day/%d/input" % [year, day]
    
    # Set up headers with cookie
    var headers: Array[String] = ["Cookie: session=%s" % session_id]
    
    # Connect with a lambda that captures http_request and file_path
    http_request.request_completed.connect(func(result: int, response_code: int, headers: PackedStringArray, body: PackedByteArray) -> void:
        if response_code == 200:
            var content: String = body.get_string_from_utf8().strip_edges()
            print("Downloaded successfully!")
            
            var file: FileAccess = FileAccess.open(file_path, FileAccess.WRITE)
            if file:
                file.store_string(content)
                file.close()
                print("Saved to: %s" % file_path)
            else:
                push_error("Failed to open file for writing: %s" % file_path)
        else:
            push_error("HTTP request failed with response code: %d" % response_code)
        
        # Remove and free the HTTPRequest node
        http_request.queue_free()
    )
    
    # Make the request
    var error: Error = http_request.request(url, headers)
    
    if error != OK:
        push_error("An error occurred in the HTTP request.")
