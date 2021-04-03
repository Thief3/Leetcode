def solution(rT):
    N = len(rT)
    drop = 0
    already_dropped = {}
    
    sec_1 = [0] # max 3
    sec_10 = [0] # max 20
    sec_60 = [0]

    last = rT[0]
    for sec in range(0, N):
        # Checking if there has been more than three in a second
        if sec > 2 and rT[sec] == rT[sec - 3]:
            if rT[sec] not in already_dropped or already_dropped[rT[sec]] != sec:
                already_dropped[rT[sec]] = sec
                drop = drop + 1
        elif sec > 19 and rT[sec] < rT[sec - 20] + 10:
            if rT[sec] not in already_dropped or already_dropped[rT[sec]] != sec:
                already_dropped[rT[sec]] = sec
                drop = drop + 1
        elif sec > 59 and rT[sec] < rT[sec - 60] + 0:
            if rT[sec] not in already_dropped or already_dropped[rT[sec]] != sec:
                already_dropped[rT[sec]] = sec
                drop = drop + 1
        
    return drop

print(solution([1,1,1,1,2]))
print(solution([1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7]))
