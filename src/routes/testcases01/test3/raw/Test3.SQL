SELECT WebBooks_02
WHERE book_type=$book_type AND[5] price>$max_price
ORDERBY price LIMIT 10 SKIP $skip
AS MathBooks

RETURN MathBooks
