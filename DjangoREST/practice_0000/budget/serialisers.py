import budget.models as models
import rest_framework.serializers as drfserials


class BudgetSerialiser(drfserials.HyperlinkedModelSerializer):
    class Meta:
        model = models.Budget
        fields = ['entry']
