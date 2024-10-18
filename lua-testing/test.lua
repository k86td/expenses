
-- this simple sccript fetches an API and decodes the JSON

print("hello from lua")

local http = require('socket.http')
local response = http.request("https://jsonplaceholder.typicode.com/todos")

local json = require('json')

print(json.decode(response))
print("decoded the JSON!")

