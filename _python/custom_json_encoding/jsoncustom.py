import json


# Custom object
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age


# Custom encoder for Person objects
class CustomEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, Person):
            return {"name": obj.name, "age": obj.age}

        return super().default(obj)

# Sample custom object
person_obj = Person("Alice", 25)

# Encoding custom object using the custom encoder
json_string = json.dumps(person_obj, cls=CustomEncoder)
print(json_string)  # Output: {"name": "Alice", "age": 25}
