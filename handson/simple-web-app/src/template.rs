pub const CATEGORY_TEMPLATE: &'static str = r#"
<!doctype html>
<html>
<head>
    <title>A Simple Web App</title>
</head>
<body>
    <h1>{{ category.title }}</h1>
    <h2>Products</h2>
    <ul>
        {% for p in category.products %}
        <li>{{ p.name }} ({{ p.list_price }} $)</li>
        {% endfor %}
    <ul>
</body>
</html>
"#;
