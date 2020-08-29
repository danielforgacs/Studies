from xmlrpc.server import SimpleXMLRPCServer
from xmlrpc.server import SimpleXMLRPCRequestHandler


def fakit():
    text = 'Heeeeyyy.... :)s'
    print(text)

    return text



class CoolClass:
    def __init__(self, pk=None):
        self.pk = pk



# Restrict to a particular path.
class RequestHandler(SimpleXMLRPCRequestHandler):
    rpc_paths = ('/RPC2',)

# Create server
with SimpleXMLRPCServer(
        ('localhost', 8000),
        requestHandler=RequestHandler,
        allow_none = True,
        use_builtin_types=True,
    ) as server:
    server.register_introspection_functions()

    # Register pow() function; this will use the value of
    # pow.__name__ as the name, which is just 'pow'.
    server.register_function(pow)
    server.register_function(fakit)
    server.register_function(CoolClass)

    # Register a function under a different name
    def adder_function(x, y):
        return x + y

    server.register_function(adder_function, 'add')

    # Register an instance; all the methods of the instance are
    # published as XML-RPC methods (in this case, just 'mul').
    class MyFuncs:
        def mul(self, x, y):
            return x * y

    server.register_instance(MyFuncs())

    # Run the server's main loop
    server.serve_forever()
