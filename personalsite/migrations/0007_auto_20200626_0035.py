# Generated by Django 3.0.6 on 2020-06-26 00:35

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ('personalsite', '0006_post_summary'),
    ]

    operations = [
        migrations.AlterField(
            model_name='post',
            name='summary',
            field=models.TextField(),
        ),
    ]
