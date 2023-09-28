DAta Relationships
 -AVOID DUPLICATION, REDUDANTION
 -Seperate your data () then use foreign key to connect your data

Relationship basics
1. One to One relationships
	ex : costumer to costumer_details (1costumer :::> 1costumer_details)
2. One to Many relationships
	ex : book to review (1 book can have thousand of reviews) (reviews belong to one book)
3. Many to Many realtionships
	ex : authors to books (books can have many authors, author can write many books)


More Explanations
- One to Many Relationships 
	ex : Costumer AND his/her Orders

CREATE TABLE customers(
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    email VARCHAR(100)
);
CREATE TABLE orders(
    id INT AUTO_INCREMENT PRIMARY KEY,
    order_date DATE,
    amount DECIMAL(8,2),
    customer_id INT,
    FOREIGN KEY(customer_id) REFERENCES customers(id)
);
SELECT * FROM customers, orders;  NOT GOOD BASICLALLY TAKING EVERY DATA IN THE CUSTOMERS TABLE
AND THEN JOIN ITSELF WITH DATA FROM THE ORDERS TABLE 

------Implicit INNER JOIN
SELECT * FROM customers, orders 
WHERE customers.id = orders.customer_id;

SELECT first_name, last_name, order_date, amount
FROM customers, orders 
    WHERE customers.id = orders.customer_id;

------Explicit INNER JOIN
SELECT first_name,last_name,order_date,amount FROM customers
JOIN orders ON customers.id =orders.customer_id;

SELECT * FROM customers
JOIN orders
    ON customers.id = orders.customer_id;
    
SELECT first_name, last_name, order_date, amount 
FROM customers
JOIN orders
    ON customers.id = orders.customer_id;
    
SELECT *
FROM orders
JOIN customers
    ON customers.id = orders.customer_id;

:::::DEFAULT INNER JOIN

SELECT first_name, last_name, order_date, AVG(amount) FROM customers 
INNER JOIN orders ON customers.id = customer_id
 GROUP BY first_name,last_name;

------LEFT JOIN ::: BASICALLY TAKING EVERYTHING WITH THE LEFT TABLE AND THE DATA THAT
EXIST ON THE BOTH TABLE
SELECT * FROM customers LEFT JOIN orders ON customers.id = customer_id;

SELECT first_name,last_name,IFNULL(SUM(amount), 0)
 FROM customers LEFT JOIN orders ON customers.id = customer_id GROUP BY customers.id;

SELECT first_name, last_name, IFNULL(SUM(amount), 0) AS "Total Amount"
 FROM customers LEFT JOIN orders ON customers.id = customer_id
 GROUP BY customers.id ORDER BY SUM(amount);

SELECT first_name, last_name, CASE
 WHEN SUM(amount) > 0 THEN SUM(amount)
 ELSE 0 
END AS "Total Amount" 
FROM customers LEFT JOIN orders ON customers.id = customer_id GROUP BY customers.id
 ORDER BY SUM(amount);


NOTE ::: YOU CANT DELETE DATA FROM A TABLE THAT HAS A DEPENDENT(FOREIGN KEY THAT REFERENCE
TO THAT TABLE THAT YOU WANT TO BE GONE), SO YOU HAVE TO DELETE THE DEPENDENT(TABLE THAT HAS
FOREIGN KEY) THAT REFERENCING TOU THE TABLE THAT YOU WANT TO BE GONE FOREVER

ON DELETE CASCADE :::
CREATE TABLE customers(
    id INT AUTO_INCREMENT PRIMARY KEY,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    email VARCHAR(100)
);
CREATE TABLE orders(
    id INT AUTO_INCREMENT PRIMARY KEY,
    order_date DATE,
    amount DECIMAL(8,2),
    customer_id INT,
    FOREIGN KEY(customer_id) REFERENCES customers(id) ON DELETE CASCADE
);

SELECT first_name, IFNULL(AVG(grade),0) AS average, 
CASE 
WHEN AVG(grade) >= 75 THEN "PASSING"
 ELSE "FAILING"
 END AS "passing status"
 FROM students
 LEFT JOIN papers ON students.id = student_id GROUP BY students.id ORDER BY AVG(grade) DESC;

SELECT first_name, 
       Ifnull(Avg(grade), 0) AS average, 
       CASE 
         WHEN Avg(grade) IS NULL THEN 'FAILING' 
         WHEN Avg(grade) >= 75 THEN 'PASSING' 
         ELSE 'FAILING' 
       end                   AS passing_status 
FROM   students 
       LEFT JOIN papers 
              ON students.id = papers.student_id 
GROUP  BY students.id 
ORDER  BY average DESC;


:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

MANY:MANY ;;; MANY TO MANY



