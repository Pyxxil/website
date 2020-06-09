from django.db import models
from martor.models import MartorField


class Post(models.Model):
    title = models.CharField(max_length=140)
    body = MartorField()
    date = models.DateTimeField()

    def __str__(self):
        return self.title
