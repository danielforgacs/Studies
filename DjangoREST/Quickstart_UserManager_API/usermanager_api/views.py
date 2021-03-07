import django.contrib.auth.models as authmodels
import rest_framework.viewsets as viewsets
import rest_framework.permissions as permissions
import usermanager_api.serialisers as serialisers





class UserViewSet(viewsets.ModelViewSet):
    queryset = authmodels.User.objects.all()
    serializer_class = serialisers.UserSerialiser
    permission_classes = [permissions.IsAuthenticated]





class GroupViewSet(viewsets.ModelViewSet):
    queryset = authmodels.User.groups.all()
    serializer_class = serialisers.GroudSerialiser
    permission_classes = [permissions.IsAuthenticated]
