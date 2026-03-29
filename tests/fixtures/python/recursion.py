# Expected metrics:
# factorial: cognitive=TBD, cyclomatic=TBD, sloc=TBD
def factorial(n):
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)
