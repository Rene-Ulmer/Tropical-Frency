import png
import sys

with open(sys.argv[1], 'rb') as fi:
    content = bytearray(fi.read())
    n_pad = 0
    while len(content) % 3:
        content.append(0)
        n_pad += 1

    with open(sys.argv[2], 'wb') as fo:
        writer = png.Writer(len(content) // 3, 1, alpha=False, compression=9)
        writer.write(fo, [content])

    with open(sys.argv[2], 'rb') as fo:
        reader = png.Reader(file=fo)
        data = list(reader.read()[2])[0].tolist()
        assert bytearray(data) == content, "Stuff was corrupted"

    print("OK, pad = {}".format(n_pad))

