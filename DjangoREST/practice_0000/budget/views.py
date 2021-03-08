import budget.serialisers as serials
import budget.models as models
import django.http as djhttp
import rest_framework.viewsets as drfviewsets
import rest_framework.permissions as drfperms


class BudgetViewSet(drfviewsets.ModelViewSet):
    queryset = models.Budget.objects.all()
    serializer_class = serials.BudgetSerialiser
    permission_classes = [drfperms.DjangoModelPermissionsOrAnonReadOnly]



def index(request):
    html = """
<script>
    console.log("--> START")

    fetch('/budgets/')
</script>
    """
    return djhttp.HttpResponse(html)
