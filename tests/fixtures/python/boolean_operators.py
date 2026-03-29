# Expected metrics:
# check_all: cognitive=TBD, cyclomatic=TBD, sloc=TBD
def check_all(a, b, c):
    if a and b and c:
        return True
    return False

# check_mixed: cognitive=TBD, cyclomatic=TBD, sloc=TBD
def check_mixed(a, b, c):
    if a and b or c:
        return True
    return False
