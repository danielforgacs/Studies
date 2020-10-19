import xmlrpc.client


server = xmlrpc.client.ServerProxy(
    uri='http://localhost:8000',
    allow_none=True,
    use_builtin_types=True,
)

stuff = [
    'pk',
    123,
    {

    }
]

response = server.do_something(stuff)
response = server.do_something(stuff)
response = server.do_something(stuff)
