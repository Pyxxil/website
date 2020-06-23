from django.http import HttpResponse
from django.template import loader
from django.core.paginator import Paginator, EmptyPage, PageNotAnInteger

from .models import Post, Project


def index(request):
    template = loader.get_template("index.html")
    return HttpResponse(template.render({}, request))


def blog(request):
    posts = Post.objects.all().order_by("-date")

    paginator = Paginator(posts, 10)
    page_number = request.GET.get("page")

    try:
        page = paginator.get_page(page_number)
    except (EmptyPage, PageNotAnInteger):
        page = paginator.get_page(1)

    template = loader.get_template("blog.html")
    return HttpResponse(template.render({"page": page}, request))


def post(request, id):
    template = loader.get_template("post.html")

    post = Post.objects.get(id=id)

    return HttpResponse(template.render({"post": post}, request))


def about(request):
    template = loader.get_template("about.html")
    return HttpResponse(template.render({}, request))


def projects(request):
    template = loader.get_template("projects.html")

    my_projects = Project.objects.all().order_by("title")

    return HttpResponse(template.render({"projects": my_projects}, request))
