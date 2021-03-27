import rest_framework.routers as drfrouters
import django.urls as urls
import django.conf.global_settings as settings
import django.conf.urls.static as urlsstatic
import budget.views as views

# print('+++', settings.STATIC_URL)
# print('+++', settings.STATIC_URL)

api_routs = drfrouters.SimpleRouter()
api_routs.register('budgets', views.BudgetViewSet)

urlpatterns = [
    urls.path('', urls.include(api_routs.urls)),
    urls.path('', views.index)
]

# print(urlsstatic)
# print(urlsstatic.static)
# print(urlsstatic.static(settings.STATIC_URL))
# ] + urlsstatic.static(settings.STATIC_URL, document_root=settings.STATIC_ROOT)
