# Battery Nagger 2
New Rust version of Battery Nagger program. The first was written
in Python using pygame and is still available.  The point
to rewrite this is to learn Rust and see how easy/hard
it would be to use Rust.  

Lesson Learned:
There is a learning curve for Rust even as an experienced
programmer. I tried it without taking a course or even
reading a book. I found out that Rust programmers think very differently than other programmers.  I am comparing them to Python, C, C++, Javascript guys.  I think the closest mental model might be C++ heavily using the template library for everything + Javascript closures (my 2 cents).

## Features 

Assumption: BN2 is setup it up as an autostart program when you log in. 

It will show you how much battery you have left.  

When it gets to the first of the user defined limits, it will start playing a ping sound.  It is a little annoying, because the pings are spaced out -- it may irritate you, but you can avoid actually taking any action because you may be otherwise occupied.  

The idea is, the spacing of the pings gives you a sense of urgency about the state of your battery capacity. This helps you manage your last remaining minutes and give you a choice as to what you may do.  Without BN2, your computer may let you know that you are at the end of your battery life -- but there is really only 1 warning and then it may shutdown for you -- bummer the choice has been made for you.

After the second limit is hit, the pings get closer together. Hopefully,
a little more annoying -- more irritating, but you might manage to 
get work done.  Although the point is, to give you a feeling that your
battery really does need power at this point in time.

Finally, when the last limit is hit, the pings are almost continuous and
when it first hits the final limit, it will say "We are now running on emergency power!".  It will also nag you by coming up to the front of the screen. There really isn't much of a choice here, plugin or die!

If nothing else, this little app has saved me many a time when I watched movies with my kids.  At times, I have forgotten to plug in to the wall, while watching the movie -- and I will hear the ping.  Given the number of pings every one is alerted to what we need to do.  Especially since the one warning that a computer normally gives you is covered by the full screen movie player.

## Gui Tool 

Using Piston with SDL2 backend


