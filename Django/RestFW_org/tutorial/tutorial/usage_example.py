import os
import io

from django.db import models

os.environ['DJANGO_SETTINGS_MODULE'] = 'tutorial.settings'

import django
django.setup()

from rest_framework import renderers
from rest_framework import parsers
import snippets.models as models
import snippets.serializers as ser


snippet1 = models.Snippet()
snippet1.code = """
de fukkin():
    print(';laksdjf')
"""
snippet1.save()

serializer1 = ser.SnippetSerializer(instance=snippet1)

print(serializer1.data)
print(serializer1.data['code'])

bin_data = renderers.JSONRenderer().render(serializer1.data)

print(bin_data)

stream = io.BytesIO(bin_data)

print(stream)

decoded_data = parsers.JSONParser().parse(stream)

print(decoded_data)



snippet1_restored = ser.SnippetSerializer(data=decoded_data)

print(snippet1_restored.is_valid())

print(snippet1_restored.validated_data)

snippet1_restored.save()

print(snippet1_restored.data)
