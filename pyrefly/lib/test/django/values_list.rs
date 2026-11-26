/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::django_testcase;

django_testcase!(
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

assert_type(Book.objects.values_list("title", "price").get(), tuple[str, Decimal])
assert_type(Book.objects.values_list("subtitle").get(), tuple[str | None])
assert_type(Book.objects.values_list("pk", "id").get(), tuple[int, int])
assert_type(Book.objects.values_list("author", "editor").get(), tuple[int, int | None])
assert_type(
    Book.objects.values_list("author_id", "author__name", "tags", "missing", F("title")).get(),
    tuple[Any, Any, Any, Any, Any],
)
assert_type(Book.objects.values_list("title", flat=True).get(), str)
assert_type(Book.objects.filter(title="x").values_list("title").first(), tuple[str] | None)
assert_type(Book.objects.values_list("title").filter(title="x").get(), tuple[str])

for title, price in Book.objects.values_list("title", "price"):
    assert_type(title, str)
    assert_type(price, Decimal)
for a, b, c in Book.objects.values_list("title", "price"):  # E: Cannot unpack tuple[str, Decimal] (of size 2) into 3 values
    pass
"#,
);

django_testcase!(
    test_values_list_rows_unchanged,
    r#"
from typing import Any, assert_type

from django.db import models

class Book(models.Model):
    title = models.CharField(max_length=100)
    price = models.DecimalField(max_digits=8, decimal_places=2)

def check(names: list[str], flag: bool) -> None:
    assert_type(Book.objects.values_list(*names).get(), Any)
    assert_type(Book.objects.values_list().get(), Any)
    assert_type(Book.objects.values_list("title", named=True).get(), Any)
    assert_type(Book.objects.values_list("title", flat=flag).get(), Any)
    assert_type(Book.objects.values_list("title", "price", flat=True).get(), Any)
"#,
);
