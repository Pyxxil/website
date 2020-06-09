from django.db import models
from django.contrib import admin

from martor.widgets import AdminMartorWidget

from .models import Post


class PostAdmin(admin.ModelAdmin):
    formfield_overrides = {
        models.TextField: {'widget': AdminMartorWidget},
    }


admin.site.register(Post, PostAdmin)
