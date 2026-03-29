# Expected metrics:
# classify: cognitive=TBD, cyclomatic=TBD, sloc=TBD
def classify(x):
    if x > 100:
        return "high"
    elif x > 50:
        return "medium"
    elif x > 0:
        return "low"
    else:
        return "negative"
