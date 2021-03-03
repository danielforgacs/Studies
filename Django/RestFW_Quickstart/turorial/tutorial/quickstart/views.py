import django.contrib.auth.models as authmodels
# from django.shortcuts import render
from rest_framework import viewsets
from rest_framework import permissions
from tutorial.quickstart.serializers import UserSerializer
from tutorial.quickstart.serializers import GroupSerializer


class UserViewSet(viewsets.ModelViewSet):
    queryset = authmodels.User.objects.all().order_by('-date-joined')
    serializer_class = UserSerializer
    permission_classes = [permissions.IsAuthenticated]



class GroupViewSet(viewsets.ModelViewSet):
    queryset = authmodels.Group.objects.all()
    serializer_class = GroupSerializer
    permission_classes = [permissions.IsAuthenticated]

