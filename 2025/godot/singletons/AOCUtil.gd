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


static func download_file(year: int, day: int, path: String) -> void:
    # Get session ID from environment variable
    var session_id: String = OS.get_environment("SESSIONID")
    
    if session_id.is_empty():
        push_error("SESSIONID environment variable not found")
        return
    
    # Create HTTPRequest node
    var http_request: HTTPRequest = HTTPRequest.new()
    # add_child(http_request)
    
    # Connect the request_completed signal
    http_request.request_completed.connect(_on_request_completed.bind(path))
    
    # Build the URL
    var url: String = "https://adventofcode.com/%d/day/%d/input" % [year, day]
    
    # Set up headers with cookie
    var headers: Array[String] = ["Cookie: session=%s" % session_id]
    
    # Make the request
    var error: Error = http_request.request(url, headers)
    
    if error != OK:
        push_error("An error occurred in the HTTP request.")

static func _on_request_completed(_result: int, response_code: int, _headers: PackedStringArray, body: PackedByteArray, file_path: String) -> void:
    if response_code == 200:
        # Convert body to string
        var content: String = body.get_string_from_utf8()
        print("Downloaded successfully!")
        print(content)
        
        # Optionally save to file
        var file: FileAccess = FileAccess.open(file_path, FileAccess.WRITE)
        if file:
            file.store_string(content)
            file.close()
            print("Saved to ", file_path)
    else:
        push_error("HTTP request failed with response code: %d" % response_code)
