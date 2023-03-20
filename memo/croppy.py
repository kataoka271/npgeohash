import itertools
import os
import sys
from glob import glob

from PIL import Image


def isblack(r, g, b, a):
    return r < 30 and g < 30 and b < 30


def crop(filename, outdir):
    img = Image.open(filename)

    x, y = 0, img.height // 2
    while isblack(*img.getpixel((x, y))):
        x += 1
    left = x

    x, y = img.width // 2, 0
    while isblack(*img.getpixel((x, y))):
        y += 1
    upper = y

    x, y = img.width - 1, img.height // 2
    while isblack(*img.getpixel((x, y))):
        x -= 1
    right = x + 1

    x, y = img.width // 2, img.height - 1
    while isblack(*img.getpixel((x, y))):
        y -= 1
    lower = y + 1

    outname = os.path.join(outdir, os.path.basename(filename))
    img.crop((left, upper, right, lower)).save(outname)

    return (filename, outname)


def main():
    filenames = sys.argv[1:-1]
    outdir = sys.argv[-1]
    if not filenames:
        print(f"Usage: {os.path.basename(sys.argv[0])} FILENAME [FILENAME...] OUTDIR")
        sys.exit(1)
    if os.path.exists(outdir) and not os.path.isdir(outdir):
        print(f"'{outdir}' is not directory.\n\nUsage: {os.path.basename(sys.argv[0])} FILENAME [FILENAME...] OUTDIR")
        sys.exit(1)
    try:
        os.makedirs(outdir, exist_ok=True)
    except OSError as e:
        print(f"{e}\n")
        print(f"Usage: {os.path.basename(sys.argv[0])} FILENAME [FILENAME...] OUTDIR")
        sys.exit(1)
    filenames = list(itertools.chain.from_iterable(map(glob, filenames)))
    for filename in filenames:
        print("cropped: {0} => {1}".format(*crop(filename, outdir)))


if __name__ == "__main__":
    main()
