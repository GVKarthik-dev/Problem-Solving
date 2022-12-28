d1,m1,y1 = (6,6,2015)
d2,m2,y2 = (9,6,2016)


fine = 0
if y2<y1:
    fine = 10_000
elif y2 == y1:
    if m2<m1 or m1> m1:
        fine = 500 * abs(m1-m2)
    elif m1 == m2:
        if d2 < d1:
            fine = 15 * abs(d2-d1)

print(fine)