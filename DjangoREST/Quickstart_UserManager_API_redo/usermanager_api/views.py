import usermanager_api.serialisers as serials
import rest_framework.viewsets as vsets
import rest_framework.permissions as permissions
import django.contrib.auth.models as authmodels



class UserViewSet(vsets.ModelViewSet):
    queryset = authmodels.User.objects.all()
    serializer_class = serials.UserSerialiser
    permission_classes = [permissions.IsAuthenticatedOrReadOnly]
