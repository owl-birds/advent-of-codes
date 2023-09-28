MYSQL
show databases;  --> showing your dabases
CREATE DATABSE <name>; ---> Creating A database nameEX = DogApp, saop_store
no white space in between word in the name of database
 BE CONSISTENT..

Dropping DATABASE
DROP DATABASE <database_name>;

USING DATABASE
USE <database_name>;

showing database that you re using
SELECT database();

TABLES IN MYSQL

DATABASE is just a bunch of tables in a relational database, at least.

CREATE TABLE table_name
(
	column_name data_type,
	column_name data_type
);
SHOW TABLES;
SHOW COLUMN FROM <table_name>;
DESC <table_name>;
DROP TABLE <table_name>;

Inserting data into table in database
INSERT INTO table_name(column_name, column_name,...) 
VALUES(value,value,...);

CREATE TABLE cats3
  (
    name VARCHAR(20) DEFAULT 'no name provided',
    age INT DEFAULT 99
  );

CREATE TABLE unique_cats2 (
    cat_id INT NOT NULL AUTO_INCREMENT,
    name VARCHAR(100),
    age INT,
    PRIMARY KEY (cat_id)
);
CREATE TABLE employees (
    id INT AUTO_INCREMENT NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    middle_name VARCHAR(255),
    age INT NOT NULL,
    current_status VARCHAR(255) NOT NULL DEFAULT 'employed',
    PRIMARY KEY(id)
);
CREATE TABLE employees (
    id INT AUTO_INCREMENT NOT NULL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    middle_name VARCHAR(255),
    age INT NOT NULL,
    current_status VARCHAR(255) NOT NULL DEFAULT 'employed'
);

CRUD Create Read Update Delete

READ
SELECT * FROM cats; 

SELECT name FROM cats; 

SELECT age FROM cats; 

SELECT cat_id FROM cats; 

SELECT name, age FROM cats; 

SELECT cat_id, name, age FROM cats; 

SELECT age, breed, name, cat_id FROM cats; 

SELECT cat_id, name, age, breed FROM cats; 

SELECT * FROM cats WHERE age=4; 

SELECT cat_id AS id, name FROM cats;
 
SELECT name AS 'cat name', breed AS 'kitty breed' FROM cats;
 
DESC cats;

UPDATE cats SET breed='Shorthair' WHERE breed='Tabby'; 

DELETE FROM cats WHERE name='Egg';
 
SELECT * FROM cats;
 
SELECT * FROM cats WHERE name='egg';
 
DELETE FROM cats WHERE name='egg';
 
SELECT * FROM cats;
 
DELETE FROM cats;
SELECT * FROM cats WHERE age=4;
 
DELETE FROM cats WHERE age=4;
 
SELECT * FROM cats WHERE age=4;
 
SELECT * FROM cats;
 
SELECT *  FROM cats WHERE cat_id=age;
 
DELETE FROM cats WHERE cat_id=age;
 
DELETE FROM cats;
 
SELECT * FROM cats;

UPDATE shirts SET color='off white', shirt_size='XS' WHERE color='white';




NEW LESSON
CREATE TABLE cats
    (
        cat_id INT NOT NULL AUTO_INCREMENT,
        name VARCHAR(100),
        age INT,
        PRIMARY KEY(cat_id)
    );
 
mysql-ctl cli
 
use cat_app;
 
source first_file.sql
 
DESC cats;
 
 
 
INSERT INTO cats(name, age)
VALUES('Charlie', 17);
 
INSERT INTO cats(name, age)
VALUES('Connie', 10);
 
SELECT * FROM cats;
 
source testing/insert.sql
SOURCE C:\Users\GL503GE\Desktop\web dummies\NEW PAGE NEW LIFE\MYSQL\1\book_data.sql
SELECT author_fname AS first, author_lname AS last, 
  CONCAT(author_fname, ' ', author_lname) AS full
FROM books;
 
SELECT author_fname AS first, author_lname AS last, 
  CONCAT(author_fname, ', ', author_lname) AS full
FROM books;
 
SELECT CONCAT(title, '-', author_fname, '-', author_lname) FROM books;
 
SELECT 
    CONCAT_WS(' - ', title, author_fname, author_lname) 
FROM books;
source book_code.sql
 
SELECT CONCAT
    (
        SUBSTRING(title, 1, 10),
        '...'
    ) AS 'short title'
FROM books;

SELECT
    SUBSTRING(REPLACE(title, 'e', '3'), 1, 10) AS 'weird string'
FROM books;

SELECT CONCAT(author_fname, REVERSE(author_fname)) FROM books;
SELECT CONCAT(author_lname, ' is ', CHAR_LENGTH(author_lname), ' characters long')
 FROM books;

SELECT CONCAT('MY FAVORITE BOOK IS ', UPPER(title)) FROM books;
 
SELECT CONCAT('MY FAVORITE BOOK IS ', LOWER(title)) FROM books;
SELECT DISTINCT CONCAT(author_fname,' ', author_lname) FROM books;
 
SELECT DISTINCT author_fname, author_lname FROM books;

SELECT title, author_fname, author_lname 
FROM books ORDER BY 1 DESC;
 
SELECT author_lname, title
FROM books ORDER BY 2;
 
SELECT author_fname, author_lname FROM books 
ORDER BY author_lname, author_fname;
SELECT title, released_year FROM books 
ORDER BY released_year DESC LIMIT 1,3;
 
SELECT title, released_year FROM books 
ORDER BY released_year DESC LIMIT 10,1;
SELECT title FROM books WHERE  title LIKE '%the';
 
SELECT title FROM books WHERE title LIKE '%the%';
SELECT title, author_fname FROM books WHERE author_fname LIKE 'da%';

LIKE '___' 3 digits num
LIKE '__' 2 digits num
LIKE '_' 1 digits num

SELECT title FROM books WHERE title LIKE '%\%%'
 
SELECT title FROM books WHERE title LIKE '%\_%'

Aggreagate Functions
SELECT COUNT(DISTINCT CONCAT(author_fname," ",author_lname)) AS "distinct author" FROM books;

SELECT COUNT(DISTINCT author_fname,author_lname) AS "distinct author" FROM books;

SELECT COUNT(title) AS "Title that contain 'the'" FROM books WHERE title LIKE "%the%";

SELECT title FROM books WHERE title LIKE '%the%';
 
SELECT COUNT(*) FROM books WHERE title LIKE '%the%';

SELECT author_lname,COUNT(author_lname) FROM books GROUP BY author_lname;

SELECT author_fname, author_lname, COUNT(*) FROM books GROUP BY author_fname,author_lname;

SELECT released_year,COUNT(*) FROM books GROUP BY released_year;

SELECT CONCAT("In ",released_year," ",COUNT(*)," book(s) released") 
AS "Insight" FROM books GROUP BY released_year;

SELECT MAX(released_year) FROM books;
SELECT MIN(released_year) FROM books;

SELECT title,pages FROM books WHERE pages = (SELECT MAX(pages) FROM books);
SELECT * FROM books 
ORDER BY pages DESC LIMIT 1;

SELECT
  author_fname,
  author_lname,
  Max(pages)
FROM books
GROUP BY author_lname,
         author_fname;
 
SELECT
  CONCAT(author_fname, ' ', author_lname) AS author,
  MAX(pages) AS 'longest book'
FROM books
GROUP BY author_lname,
         author_fname;

SELECT CONCAT(author_fname," ",author_lname),SUM(pages) 
FROM books GROUP BY author_fname,author_lname;

SELECT released_year, AVG(stock_quantity) 
FROM books 
GROUP BY released_year;
 
SELECT author_fname, author_lname, AVG(pages) FROM books
GROUP BY author_lname, author_fname;

SELECT pages, CONCAT(author_fname, ' ', author_lname) FROM books
ORDER BY pages DESC;
 
SELECT released_year AS year,
    COUNT(*) AS '# of books',
    AVG(pages) AS 'avg pages'
FROM books
    GROUP BY released_year;

DATA types 
-CHAR(store text) : Has a fixed length : ex : CHAR(3) Only 3 char allowed
CHAR is faster for fixed length text EX : State Abbreviations, Yes/no Flags, SEX(M/F)
-DECIMAL MORE PRECISE FIXED VALUES DECIMAL(5,2) 5 digits with 2 num behind coma
USING DECIMAL YOU CAN BE MORE PRECISE AND HAVE MORE CONTROL ON YOUR NUMBER

-float will have an issue when num >= 7 digits 4 bytes APPROXIMATE VALUES FLOAT
-double will have an issue when num >= 15 digits 8 bytes APPROXIMATE VALUES DOUBLE

-DATETIME 'YYYY-MM-DD HH:MM:SS' FORMAT
CREATE TABLE people (name VARCHAR(100), birthdate DATE, birthtime TIME, birthdt DATETIME);

INSERT INTO people(name,birthdate,birthtime,birthdt) VALUE("owl4",CURDATE(),CURTIME(),NOW());
CURDATE() DATE NOW
CURTIME() TIME NOW
NOW() DATE TIME NOW

SELECT DATE_FORMAT(birthdt, 'Was born on a %W') FROM people;
 
SELECT DATE_FORMAT(birthdt, '%m/%d/%Y') FROM people;
 
SELECT DATE_FORMAT(birthdt, '%m/%d/%Y at %h:%i') FROM people;

SELECT birthdt,birthdt + INTERVAL 10 MONTH + INTERVAL 10 HOUR FROM people;


SELECT DATEDIFF(NOW(), birthdate) FROM people;
 
SELECT name, birthdate, DATEDIFF(NOW(), birthdate) FROM people;

CREATE TABLE comments2 (
    content VARCHAR(100),
    changed_at TIMESTAMP DEFAULT NOW() ON UPDATE NOW()
); time will update when the data is updated (changed)

TIMESTAMP have smaller range then DATETIME

SELECT DATE_FORMAT(NOW(), '%w') + 1;
 
SELECT DAYNAME(NOW());
SELECT DATE_FORMAT(NOW(), '%W');
 
SELECT DATE_FORMAT(CURDATE(), '%m/%d/%Y');
 
SELECT DATE_FORMAT(NOW(), '%M %D at %h:%i');

CREATE TABLE tweets(
    content VARCHAR(140),
    username VARCHAR(20),
    created_at TIMESTAMP DEFAULT NOW()
);

Logical Operators :::: = != > < >= <= :::: AND &&(&& soon will be removed) , 
OR ||(|| soon will be removed) in the later version 
SELECT released_year,title FROM books 
WHERE released_year>2000 AND released_year < 2010 ORDER BY released_year;

SELECT released_year FROM books WHERE released_year NOT LIKE "%2%";

SELECT title,pages FROM books WHERE pages BETWEEN 200 AND 400;
SELECT title,pages FROM books WHERE pages NOT BETWEEN 200 AND 400;

SELECT 
    name, 
    birthdt 
FROM people
WHERE 
    birthdt BETWEEN CAST('1980-01-01' AS DATETIME)
    AND CAST('2000-01-01' AS DATETIME);

SELECT title,author_lname FROM books 
WHERE author_lname = "Carver" OR author_Lname = "Lahiri";
 ====================================== SAMA ===================================
SELECT title,author_lname FROM books WHERE author_lname IN("Carver","Lahiri");

SELECT title,released_year FROM books WHERE released_year > 2000 
AND released_year NOT IN(2000,2002,2004,2006,2008,2010,2012,2014,2016);


SELECT title,released_year FROM books WHERE released_year != 2000 AND released_year != 2002 
AND released_year != 2004 AND released_year != 2006 AND released_year != 2008 AND released_year
!= 2010 AND released_year != 2012 AND released_year != 2014 AND released_year != 2016;
================= samaa ===============
SELECT title,released_year FROM books WHERE released_year 
NOT IN(2000,2002,2004,2006,2008,2010,2012,2014,2016);


OPERATOR IN MYSQL ::: MODULO % (REMAINDER)
SELECT title,released_year FROM books WHERE released_year%2 != 0;

SELECT SUBSTRING(title,1,10) AS "short title",released_year, 
	CASE 
		WHEN released_year >= 2000 THEN "Modern Lit" 
		ELSE "20th Century lit" 
	END AS "Genre"
FROM books;

SELECT title,stock_quantity,
CASE 
	 WHEN stock_quantity BETWEEN 0 AND 50 THEN "*"
	 WHEN stock_quantity BETWEEN 51 AND 100 THEN "**"
	 ELSE "***" 
END AS "stock"
FROM books;

SELECT title,stock_quantity,
CASE
	WHEN stock_quantity <= 50 THEN "*"
	WHEN stock_quantity <= 100 THEN "**" 
	ELSE "***"
END AS "stock"
FROM books;

SELECT title, 
CASE 
	WHEN title LIKE "%stories%" THEN "Short Stories"
	 WHEN title LIKE "Just Kids%" OR title LIKE "A Heartbreaking Work%" THEN "Memoir"
	 ELSE "Novel"
 END AS "TYPE" 
FROM books;





