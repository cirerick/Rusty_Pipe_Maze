# Rusty_Pipe_Maze
Basically, an example of Pipe Maze, a mini-game from the first Mario Party game. The mini-game compromised of a maze, where the maze had four inlets and four outlets all interconnected by multiple bridges. Bridges would connect two of the four pipes, never overlapping and always adjacent. The player would insert a chest into any of the four inlets, the chest then would follow the flow downwards, crossing each bridge it meets until it reaches an outlet.

<img style="width: 27%;" src="https://github.com/cirerick/Rusty_Pipe_Maze/blob/master/misc/pipemazemp1_ss.png" alt="Insert_PipeMaze_Screenshot_Here."/>
<sup>Image source: https://www.youtube.com/watch?v=0lqo16GvcMw&ab_channel=Nintendo64Movies</sup>
<br/>
<br/>
My main goal here was to create a 1:1 memory layout of Pipe Mazes physical appearance. Where each node/pipe has a crossing point (bridging) with another node/pipe. I use the terminal to draw out the game, where “O” are the nodes and “---” are the bridges connecting each node. The “♥” would be the player and “⋆” is the goal to reach.

<img src="https://github.com/cirerick/Rusty_Pipe_Maze/blob/master/misc/rustypipes_democlip.gif" width="30%"/>

Although linked lists are highly advised against (especially doubly linked list), I believed this data structure was the best way to demonstrate connectivity throughout the maze. The program will follow where the player pointer as it traverses appointing to each node, where the node appoints to some memory address.  
The program can be easily ran by cloning the repository, going into the directory where it was cloned and typing into the terminal `cargo run`.