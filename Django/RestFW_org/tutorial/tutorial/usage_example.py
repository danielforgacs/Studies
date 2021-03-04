import os

os.environ['DJANGO_SETTINGS_MODULE'] = 'tutorial.settings'

import django
django.setup()

from rest_framework import renderers
from rest_framework import parsers
import snippets.models as mod
import snippets.serializers as ser


snippet1 = mod.Snippet()
snippet1.code = """
de fukkin():
    print(';laksdjf')
"""
snippet1.save()

serializer1 = ser.SnippetSerializer(instance=snippet1)

print(serializer1.data)
print(serializer1.data['code'])

content = renderers.JSONRenderer().render(serializer1.data)

print(content)
