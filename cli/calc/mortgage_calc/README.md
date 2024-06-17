# Amortization Mortgage Calculator for the Netherlands

### To run the calculation:

```bash
cargo run -- -p 777000 -r 4.50 -t 30 -w 840000 -i 100000 --period Monthly --mortgage-type Linear
cargo run -- -p 766566.67 -r 3.90 -t 354 -w 840000 -i 100000 --period Monthly --mortgage-type Annuity
```



- p = Sets the principal loan amount
- r = Sets the annual interest rate (in percentage, e.g., 5 for 5%)
- t = Sets the loan term in years
- w = Sets the total worth of the house (woz worth)
- i = Sets the highest earning income (necessary for tax calc (if above 73032 additional tax has to be paid))
- --period = Yearly or Monthly
- --mortgage-type = Linear or Annuity
d