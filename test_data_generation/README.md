
Test Data Generation
====================

Generate a csv for use in the data_cleaning exercise. 

# Header: 
transaction_id,cust_id,tran_date,prod_subcat_code,prod_cat_code,Qty,Rate,Tax,total_amt,Store_type

# Usage:
```bash
cargo run | out-file example.csv -encoding utf8
```
- Note it is important the output CSV file is UTF-8 encoded to work with the data_cleaning rust application