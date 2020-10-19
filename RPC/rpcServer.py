from xmlrpc.server import SimpleXMLRPCServer
from xmlrpc.server import SimpleXMLRPCRequestHandler



def do_something(stuffs):
    print(stuffs)



class RequestHandler(SimpleXMLRPCRequestHandler):
    rpc_paths = ('/RPC2',)



with SimpleXMLRPCServer(
        addr=('localhost', 8000),
        requestHandler=RequestHandler,
        allow_none = True,
        use_builtin_types=True,
    ) as server:

    server.register_introspection_functions()
    server.register_function(do_something)
    server.serve_forever()
