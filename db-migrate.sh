#!/bin/bash

CREATE TABLE orders (id SERIAL, total DOUBLE PRECISION, currency VARCHAR, status VARCHAR);
INSERT INTO orders VALUES (123, 30.00, 'USD', 'shipped');
INSERT INTO orders VALUES (124, 20.00, 'USD', 'processing');

