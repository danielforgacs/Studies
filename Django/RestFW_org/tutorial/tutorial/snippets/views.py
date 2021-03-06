# from django.shortcuts import render
import snippets
import django.http as http
import django.views.decorators as decorators
import rest_framework.parsers as parsers
import snippets.models as models
import snippets.serializers as serializers




@decorators.csrf_exempt
def snippet_list(request):
    if request.method == 'GET':
        snippets = models.Snippet.objects.all()
        serializer = serializers.SnippetModelSerializer(data=snippets, many=True)
        response = http.JsonResponse(serializer.data, safe=False)

    elif request.method == 'POST':
        data = parsers.JSONParser(request)
        serializer = serializers.SnippetModelSerializer(data=data)

        if serializer.is_valid():
            serializer.save()

            return http.JsonResponse(serializer.data, status=201)

        return http.JsonResponse(serializer.errors, status=400)
