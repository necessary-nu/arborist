# Expected metrics:
# apply_filter: cognitive=TBD, cyclomatic=TBD, sloc=TBD
def apply_filter(items):
    threshold = 10
    filtered = list(filter(lambda x: x > threshold, items))
    return filtered
