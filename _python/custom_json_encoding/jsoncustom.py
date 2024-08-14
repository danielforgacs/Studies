import json


class A:
    pass
    def as_dict(self):
        return {'a': 1}



a = A()
data = json.dumps(a.as_dict())
print(data)
