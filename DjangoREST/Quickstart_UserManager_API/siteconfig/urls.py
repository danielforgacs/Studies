"""siteconfig URL Configuration

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/3.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
import rest_framework.routers as routers
import django.urls as urls
import usermanager_api.views as views
from django.contrib import admin


api_routers = routers.DefaultRouter()
api_routers.register('users', views.UserViewSet)
api_routers.register('groups', views.GroupViewSet)

urlpatterns = [
    urls.path('', urls.include(api_routers.urls)),
    # urls.path('api-auth/', urls.include('rest_framework.urls', namespace='rest_framework')),
    # urls.path('admin/', admin.site.urls),
]
