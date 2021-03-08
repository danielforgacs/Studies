import budget.serialisers as serials
import rest_framework.viewsets as drfviewsets
import rest_framework.permissions as drfperms
import budget.models as models


class BudgetViewSet(drfviewsets.ModelViewSet):
    queryset = models.Budget.objects.all()
    serializer_class = serials.BudgetSerialiser
    permission_classes = [drfperms.DjangoModelPermissionsOrAnonReadOnly]
