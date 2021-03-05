import os

os.environ['DJANGO_SETTINGS_MODULE'] = 'tutorial.settings'

import django

django.setup()

import snippets.serializers as serializers
import snippets.models as models


snippetserial = serializers.SnippetModelSerializer()

print(snippetserial)
