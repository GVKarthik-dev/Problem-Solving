s = 'aaabccddd'


def test(s):
    result = ''
    if len(s) == 0:
        return 'Empty String'
    else:
        
        for i in range(len(s)-1):
            if s[i] == s[i+1]:
                s = s[:i] +s[i+2:]
            else:
                break

    return test(s)


test(s)