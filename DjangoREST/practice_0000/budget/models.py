import django.db.models as models


class Budget(models.Model):
    entry = models.FloatField()
