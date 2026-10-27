#!/usr/bin/env python3

"""Produces logs in nginx combined format for testing purposes.
Usage: python3 tools/logs_gen.py 500000 > samples/big.log
"""
import random
import sys
from datetime import datetime, timedelta

N = int(sys.argv[1]) if len(sys.argv) > 1 else 1000
random.seed(42)
PATHS = [
    ("/", 0.02), ("/api/products", 0.05), ("/api/products/42", 0.04),
    ("/api/cart", 0.08), ("/api/checkout", 0.45), ("/api/search?q=rust", 0.25),
    ("/login", 0.06), ("/static/app.js", 0.01), ("/static/style.css", 0.01),
    ("/api/reports/monthly", 1.2),
]

METHODS = ["GET"] * 8 + ["POST"] * 3 + ["PUT", "DELETE"]
STATUSES = [200] * 40 + [201, 204, 301, 304, 304] + [400, 401, 403, 404, 404, 404] + [500, 502, 503]
AGENTS = [
    "Mozilla/5.0 (X11; Linux x86_64) Firefox/131.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_6) Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/130.0",
    "curl/8.9.1",
    "Googlebot/2.1 (+http://www.google.com/bot.html)",
]

IPS = [f"10.0.{random.randint(0, 20)}.{random.randint(1, 254)}" for _ in range(300)]

# Gerçekçi bir gün: gece sakin, öğlen ve akşam yoğun
HOUR_WEIGHTS = [2, 1, 1, 1, 1, 2, 4, 7, 10, 12, 13, 14, 15, 14, 13, 12, 12, 13, 14, 12, 9, 7, 5, 3]
day = datetime(2026, 10, 25)
hours = random.choices(range(24), weights=HOUR_WEIGHTS, k=N)
times = sorted(day + timedelta(hours=h, seconds=random.randint(0, 3599)) for h in hours)
out = sys.stdout

for t in times:
    path, base = random.choice(PATHS)
    status = random.choice(STATUSES)
    size = "-" if status in (204, 304) else str(random.randint(200, 50_000))
    rt = max(0.001, random.gauss(base, base / 3))
    if status >= 500:
        rt *= 4

    out.write(
        f'{random.choice(IPS)} - - [{t.strftime("%d/%b/%Y:%H:%M:%S")} +0300] '
        f'"{random.choice(METHODS)} {path} HTTP/1.1" {status} {size} "-" '
        f'"{random.choice(AGENTS)}" {rt:.3f}\n'
    )
