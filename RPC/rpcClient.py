import xmlrpc.client

server = xmlrpc.client.ServerProxy(
    uri='http://localhost:8000',
    allow_none=True,
    use_builtin_types=True,
)
print(server.system.listMethods())

response = server.fakit()
print(response)

print(type(server.CoolClass))

insta = server.CoolClass()
print(type(insta))
# insta2 = server.CoolClass(pk=1)
print(insta)
# print(insta2.pk)
