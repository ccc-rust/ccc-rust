def fixPoint(f, x, dist, gap = 0.0000001):
    while (True):
        fx = f(x)
        print('x=', x, 'fx=', fx)
        if dist(x-fx) < gap:
            break
        x = fx

    return x
