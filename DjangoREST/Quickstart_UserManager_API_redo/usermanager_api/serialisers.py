import rest_framework.serializers as drfseri
import django.contrib.auth.models as authmodels





class UserSerialiser(drfseri.HyperlinkedModelSerializer):
    class Meta:
        model = authmodels.User
        fields = ['username', 'email', 'id', 'pk', 'url']
