import rest_framework.serializers as serializers
import django.contrib.auth.models as authmodels





class UserSerialiser(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = authmodels.User
        fields = ['url', 'username', 'email', 'groups']





class GroudSerialiser(serializers.HyperlinkedModelSerializer):
    class Meta:
        model = authmodels.Group
        fields = ['url', 'name']
