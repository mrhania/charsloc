charsloc
========

This library simply extends your character iterator with file location
information. Yep, this is pretty much eveything it does.

The library provides two iterator wrappers: `Located` and `Tagged`. `Located` is just an iterator that lets you query it for current location. `Tagged` on the other hands transforms an iterator of `char`s to iterator of `(char, Location)`.

Refer to the documentation for more information and usage samples.
