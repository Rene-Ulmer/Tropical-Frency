#!/usr/bin/env python
import png
import sys

def format_as_include(name, width, height, data):
    # Try to make a valid constant name out of the file path.
    name = name.upper().replace(".PNG", "")

    # Strip path components
    if name.find("/") != -1:
        name = name[name.rfind("/") + 1:]
    if name.find("\\") != -1:
        name = name[name.rfind("\\") + 1:]

    assert width * height * 3 == len(data), "?! bug in the program :<"

    # Format it as u8 include.
    print("const {}: &'static [u8; {} * {} * {}] = &{};".format(
        name, width, height, 3, data
    ))


if __name__ == '__main__':
    assert len(sys.argv) == 2, "usage: {} <image.png>".format(sys.argv[0])
    with open(sys.argv[1], 'rb') as fi:
        reader = png.Reader(file=fi)

        try:
            (width, height, weird, _) = reader.asRGB8()

            data = []
            for elem in list(weird):
                data.extend(elem.tolist())
        except:
            print("Unable to load image, incorrect file / has alpha channel?")
            raise

        format_as_include(sys.argv[1], width, height, data)
