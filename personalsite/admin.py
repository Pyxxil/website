from django.db import models
from django.contrib import admin

from martor.widgets import AdminMartorWidget

from .models import Post, Project

from .utils import read_time


class PostAdmin(admin.ModelAdmin):
    formfield_overrides = {
        models.TextField: {"widget": AdminMartorWidget},
    }
    readonly_fields = ('reading_time',)

    def save_model(self, request, obj, form, change):
        obj.reading_time = read_time(obj.body)
        super().save_model(request, obj, form, change)


class ProjectAdmin(admin.ModelAdmin):
    formfield_overrides = {
        models.TextField: {"widget": AdminMartorWidget},
    }


admin.site.register(Post, PostAdmin)
admin.site.register(Project, ProjectAdmin)
