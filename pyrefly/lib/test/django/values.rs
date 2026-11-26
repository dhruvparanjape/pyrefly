/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::django_testcase;

django_testcase!(
    bug = "values() row values are Any",
    test_values_rows,
    r#"
from typing import Any, assert_type

from django.db import models
from django.db.models import F

class Author(models.Model):
    name = models.CharField(max_length=100)

class Book(models.Model):
    title = models.CharField(max_length=100)
    subtitle = models.CharField(max_length=100, null=True)
    price = models.DecimalField(max_digits=8, decimal_places=2)
    author = models.ForeignKey(Author, on_delete=models.CASCADE)

row = Book.objects.values("title", "subtitle", "price", "pk", "author", "author__name", upper=F("title")).get()
assert_type(row["title"], Any)
assert_type(row["subtitle"], Any)
assert_type(row["price"], Any)
assert_type(row["pk"], Any)
assert_type(row["author"], Any)
assert_type(row["author__name"], Any)
assert_type(row["upper"], Any)

first = Book.objects.filter(title="x").values("title").first()
if first is not None:
    assert_type(first["title"], Any)
for sliced in Book.objects.values("title").order_by("title")[:5]:
    assert_type(sliced["title"], Any)
rows: list[dict[str, Any]] = list(Book.objects.values("title", "price"))
"#,
);

django_testcase!(
    test_values_rows_unchanged,
    r#"
from typing import Any, assert_type

from django.db import models
from django.db.models import F

class Book(models.Model):
    title = models.CharField(max_length=100)

def check(names: list[str]) -> None:
    assert_type(Book.objects.values().get()["title"], Any)
    assert_type(Book.objects.values(*names).get()["title"], Any)
    assert_type(Book.objects.values("title", F("title")).get()["title"], Any)
"#,
);

django_testcase!(
    bug = "values() row values are Any",
    test_values_then_annotate,
    r#"
from typing import Any, assert_type

from django.db import models
from django.db.models import Count

class Book(models.Model):
    title = models.CharField(max_length=100)

keyword = Book.objects.values("title").annotate(n=Count("id")).get()
assert_type(keyword["title"], Any)
assert_type(keyword["n"], Any)
positional = Book.objects.values("title").annotate(Count("id")).get()
assert_type(positional["title"], Any)
assert_type(positional["id__count"], Any)
"#,
);
