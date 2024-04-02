# Raylib Window Procedure interception (Window Subclassing)

If you have used raylib before, you might be familiar with how undecorated windows remove the default resizing/docking behaviour of a window on the Windows OS.  

This project suddenly came about when I was researching a way to add such default behaviour back to a window. The subclassing was successful! So luckily I learnt something with this project.  

However, my actual goal of re-implementing the default behaviour of window resizing & docking didn't work. My theory is that this is due to some flag that raylib specifies on the backend during window creation, which cannot be changed, and I do not have enough knowledge of the raylib repo to be able to fix it myself.  

I'm uploading this in hopes that maybe the code could be useful to someone out there, trying to fix a similar problem. The code is pretty simple, so I'm sure anyone could understand and modify this to their own needs.  

If you manage to find a way to re-implement this default window behaviour, with or without my code, please contact me!! I'd love to see how you did it.
