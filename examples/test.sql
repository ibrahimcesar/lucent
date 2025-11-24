-- SQL example file

/*
 * Multi-line comment
 * Database schema and functions
 */

-- Create a simple function
CREATE OR REPLACE FUNCTION calculate_discount(price DECIMAL, rate DECIMAL)
RETURNS DECIMAL
AS $$
BEGIN
    RETURN price * (1 - rate / 100);
END;
$$ LANGUAGE plpgsql;

-- Create a procedure
CREATE PROCEDURE update_inventory(product_id INT, quantity INT)
BEGIN
    UPDATE products
    SET stock = stock + quantity
    WHERE id = product_id;
END;

-- Another function
CREATE FUNCTION get_customer_count()
RETURNS INT
AS $$
    SELECT COUNT(*) FROM customers;
$$ LANGUAGE SQL;
