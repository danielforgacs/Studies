# from django.shortcuts import render
# import snippets
import django.http as http
# import django.views.decorators as decorators
import django.views.decorators.csrf as csrf
import rest_framework.parsers as parsers
import snippets.models as models
import snippets.serializers as serializers




@csrf.csrf_exempt
def snippet_list(request):
    if request.method == 'GET':
        snippets = models.Snippet.objects.all()
        serializer = serializers.SnippetModelSerializer(snippets, many=True)
        response = http.JsonResponse(serializer.data, safe=False)

        return response

    elif request.method == 'POST':
        data = parsers.JSONParser(request)
        serializer = serializers.SnippetModelSerializer(data=data)

        if serializer.is_valid():
            serializer.save()

            return http.JsonResponse(serializer.data, status=201)

        return http.JsonResponse(serializer.errors, status=400)



@csrf.csrf_exempt
def snippet_detail(request, pk):
    try:
        snippet = models.Snippet.objects.get(pk=pk)
    except models.Snippet.DoesNotExist as error:
        return http.HttpResponse(status=404)

    if request.method == 'GET':
        serializer = serializers.SnippetModelSerializer(snippet)

        return http.JsonResponse(serializer.data)

    elif request.method == 'PUT':
        data = parsers.JSONParser().parse(request)
        serializer = serializers.SnippetSerializer(data=data)

        if serializer.is_valid:
            serializer.save()

            return http.JsonResponse(serializer.data)
        return http.JsonResponse(serializer.errors, status=400)

    elif request.method == 'DELETE':
        snippet.delete()

        return http.HttpResponse(status=204)
