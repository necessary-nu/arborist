# Expected metrics:
# process: cognitive=TBD, cyclomatic=TBD, sloc=TBD
def process(items):
    total = 0
    if len(items) > 0:
        for item in items:
            if item > 0:
                total += item
    return total
