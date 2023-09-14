# Amortization Mortgage Calculator for the Netherlands

### To run the calculation:

```bash
cargo run -- -p 785000 -r 4.65 -t 30 -w 840000 -i 95000
```

- p = Sets the principal loan amount
- r = Sets the annual interest rate (in percentage, e.g., 5 for 5%)
- t = Sets the loan term in years
- w = Sets the total worth of the house (woz worth)
- i = Sets the highest earning income (necessary for tax calc)
