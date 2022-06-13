# Restaurant API
Handles requests related to placing orders for tables at a restaurant

## PostgreSQL Setup
```
#setting up postgres
psql -U postgres -h localhost
CREATE DATABASE restaurant;
CREATE DATABASE restaurant_test;
\q
```

## Running
```
#running server
cargo run
```

## Testing
```
#testing server
cargo test
```

## Requests
```
# create order, returns order json
curl -X POST localhost:8080/order -H 'content-type: application/json' -d '{"item_name":"burrito", "table_id": 1}'

# get order, returns order json (may not have id of 1)
curl -X GET localhost:8080/order -H 'content-type: application/json' -d '{"id":1, "table_id": 1}'

# get orders for table, returns json array of orders
curl -X GET localhost:8080/table -H 'content-type: application/json' -d '{"table_id": 1}'

# delete order
curl -X DELETE localhost:8080/order -H 'content-type: application/json' -d '{"id":1, "table_id": 1}'
```

