print(2)
m = 999999
m_h = int(m/2)
for j in range(2):
    print(1000000000, m_h, m_h)
    b = [];
    a = [];
    for i in range(0, m):
        if (i % 2 == 0):
            b.append(i)
        else:
            a.append(i)
    
    for i in range(0, m_h):
        print(b[i], end=" ")
    print("");

    for i in range(0, m_h):
        print(a[i], end=" ")
    print("");
