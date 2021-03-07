import django.urls as urls
import snippets.views as views

urlpatterns = [
    urls.path('snippets/', views.snippet_list),
    urls.path('snippets/<int:pk>', views.snippet_detail),
]
