/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::django_testcase;

django_testcase!(
    bug = "values_list() rows are Any",
    test_values_list_rows,
    r#"
from decimal import Decimal
from typing import Any, assert_type

from django.db import models
from django.db.models import F

class Author(models.Model):
    name = models.CharField(max_length=100)

class Tag(models.Model):
    label = models.CharField(max_length=20)

class Book(models.Model):
    title = models.CharField(max_length=100)
    subtitle = models.CharField(max_length=100, null=True)
    price = models.DecimalField(max_digits=8, decimal_places=2)
    author = models.ForeignKey(Author, on_delete=models.CASCADE)
    editor = models.ForeignKey(Author, on_delete=models.CASCADE, null=True, related_name="+")
    tags = models.ManyToManyField(Tag)

assert_type(Book.objects.values_list("title", "price").get(), Any)
assert_type(Book.objects.values_list("subtitle").get(), Any)
assert_type(Book.objects.values_list("pk", "id").get(), Any)
assert_type(Book.objects.values_list("author", "author_id").get(), Any)
assert_type(Book.objects.values_list("editor").get(), Any)
assert_type(Book.objects.values_list("author__name", "tags", "missing", F("title")).get(), Any)
assert_type(Book.objects.values_list("title", flat=True).get(), Any)
assert_type(Book.objects.annotate(extra=F("title")).values_list("extra", "title").get(), Any)
assert_type(Book.objects.filter(title="x").values_list("title").first(), Any | None)
assert_type(Book.objects.values_list("title").filter(title="x").get(), Any)
assert_type(Book.objects.values_list("title", named=True).get(), Any)
assert_type(Book.objects.values_list().get(), Any)

def star(names: list[str]) -> None:
    assert_type(Book.objects.values_list(*names).get(), Any)

for title, price in Book.objects.values_list("title", "price"):
    assert_type(title, Any)
    assert_type(price, Any)
"#,
);
