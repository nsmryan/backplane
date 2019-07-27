# BackPlane
This library is a work in progress! It is being published to integrate in other
projects, to get an idea of an appropriate API.


BackPlane is a simple wrapper over several interfaces (files, TCP, UDP, and hopefully
eventually serial) to help with tools that move data between interfaces.


The primary purpose is to use with the ccsds_router tool (https://github.com/nsmryan/CCSDS-Router),
but the concept could extend to a separate command line program and a library
for moving and splitting data streams between common interfaces.


This is not intended to be the highest speed solution- it is intended to make certain
tools simple.


## The Name
The name blackplane was chosen to evoke the image of a series of connected components,
each interfacing with a single backplane, and passing messages around. While this
is not a messaging system, it does provide a routing system for moving data
between streams.
