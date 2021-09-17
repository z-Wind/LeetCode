# Write your MySQL query statement below
Select FirstName, LastName, City, State 
FROM Person
LEFT JOIN Address
ON Person.PersonId = Address.PersonId;