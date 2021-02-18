# OrdersCtl

OrdersCtl is a (for now) dumb CLI to help me track my orders.

It read everything from a file with items such as:

```
- description: "Awesome coffee grinder"
  provider: "Amazon"
  id: "12345ABCDEFG"
  date: "2021-01-31"
  content:
    - "1x Super Mega Coffee Grinder"
    - "1x Coffee Beans 500g"
  cost: 42 # USD
  status:
    confirmed: true
    paid: true
    shipped: false
    received: false
```

... and then prints it as a table:

```
+--------------------------------------+---------------+---------------------+------------+----------+-----------+------+---------+----------+
| Description                          | Provider      | Order ID            | Date       | Cost     | Confirmed | Paid | Shipped | Received |
+--------------------------------------+---------------+---------------------+------------+----------+-----------+------+---------+----------+
| Awesome coffee grinder               | Amazon        | 12345ABCDEFG        | 2021-01-31 | €42      |    ✅     |  ✅  |   ❌    |    ❌    |
+--------------------------------------+---------------+---------------------+------------+----------+-----------+------+---------+----------+
```
