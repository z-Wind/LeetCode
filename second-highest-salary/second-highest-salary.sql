# Write your MySQL query statement below
Select Max(Salary) as SecondHighestSalary
From Employee
WHERE Salary < ( SELECT MAX( Salary ) FROM Employee )