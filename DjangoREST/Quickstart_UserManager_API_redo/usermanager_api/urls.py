import django.urls as urls
import usermanager_api.views as views
import rest_framework.routers as routers


# api_routs = routers.SimpleRouter()
api_routs = routers.DefaultRouter()
api_routs.register('users', views.UserViewSet)


print('--> api routes', api_routs)
print('--> api routes urls\n%s' % '\n'.join(map(str, api_routs.urls)))
print()



urlpatterns = [
    # urls.path('api-auth/', urls.include('rest_framework.urls')),
    # urls.path(route='', view=views.UserViewSet, name='user')
    urls.path(route='', view=urls.include(api_routs.urls)),
]
