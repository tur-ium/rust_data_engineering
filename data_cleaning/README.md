Project 1: Data Cleaning in Rust
=====================

# Objectives
The objective of this notebook is to detail each of these two steps in order to obtain a clean and easily usable DataFrame.

## 1. Load data
1. Load a CSV as a DataFrame
2. Display the data

## Manage data in a DataFrame
1. Check for duplicates, and clean them up if they exists
2. Modify of the elements of a DataFrame (replace, rename and astype methods)
3. Operations on the values of a DataFrame (apply method and lambda functions)

## Deal with missing values
A missing value is either:
- An unspecified value.
- A value that does not exist. In general, they result from mathematical calculations having no solution (a division by zero for example).

In this part, we will see several methods to:
1. Detect missing values (isna and any methods)
2. Replace these values (fillna method)
3. Delete missing values (dropna method)
