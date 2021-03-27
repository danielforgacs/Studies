import budget.serialisers as serials
import budget.models as models
import django.http as djhttp
import django.shortcuts as djshort
import rest_framework.viewsets as drfviewsets
import rest_framework.permissions as drfperms


class BudgetViewSet(drfviewsets.ModelViewSet):
    queryset = models.Budget.objects.all()
    serializer_class = serials.BudgetSerialiser
    permission_classes = [drfperms.DjangoModelPermissionsOrAnonReadOnly]



def index(request):
    response = djshort.render(request=request, template_name='budget/index.html', context={})

    return response
