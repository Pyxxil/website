from django.db import models
from martor.models import MartorField


class Post(models.Model):
    title = models.CharField(max_length=140)
    summary = models.TextField()
    body = MartorField()
    date = models.DateTimeField()
    reading_time = models.IntegerField(default=0)

    def __str__(self):
        return self.title


class Project(models.Model):
    title = models.CharField(max_length=140)
    link = models.CharField(max_length=200)
    description = MartorField()

    def __str__(self):
        return f"{self.title}: {self.link}"
