
reports = [[int(y) for y in x.split()]
           for x in open('../input.txt').read().split('\n')]


def report_is_valid(report):
    windows = zip(report, report[1:])
    return all(w[0] < w[1] and abs(w[0] - w[1]) < 4 for w in windows) \
        or all(w[0] > w[1] and abs(w[0] - w[1]) < 4 for w in windows)


sum = 0
for report in reports:
    if report_is_valid(report):
        sum += 1
    else:
        for i in range(len(report)):
            clone = report.copy()
            clone.pop(i)
            if report_is_valid(clone):
                sum += 1
                break

print(sum)
