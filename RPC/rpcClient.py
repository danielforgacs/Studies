import xmlrpc.client

server = xmlrpc.client.ServerProxy('http://localhost:8000')
print(server.system.listMethods())

response = server.fakit()
print(response)
