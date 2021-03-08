import rest_framework.routers as drfrouters
import django.urls as urls
import budget.views as views


api_routs = drfrouters.SimpleRouter()
api_routs.register('budgets', views.BudgetViewSet)

urlpatterns = [
    urls.path('', urls.include(api_routs.urls)),
]
