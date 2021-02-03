# png-find-color

Defines a function that looks for the first pixel which
isn't transparent, along the bottom row of a PNG.  The
result is returned as a percentage of the image's width.

This trivial program supports [issue 38 of Terkwood/forest](https://github.com/Terkwood/forest/issues/38).
It helps us center trees which are drawn such that their
trunk is not perfectly centered in the image.
