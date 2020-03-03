# Crabby Jump
###### [aka…. Froggy Jump… but in Rust]

<img src=”froggy_jump.jpg style=”height: 400px; width: auto;”/>

## What We Plan to Build

We are building a game that hopefully emulates some of the basics of the game Froggy Jump (shown above). In addition, we also want to make this a networked multiplayer game, so that a person may play with other people*. 

########### *At least one as a basic requirement, more than one as a reach goal

### Must-Have Requirements
* Player controls a constantly jumping crab
* Crab can be moved left or right using arrow keys
* Random platforms are generated for the user to jump on
* Player starts by jumping on a designated platform and loses when they fall
* Players are scored based on height reached
* Player can select between single-player and multiplayer
* Multiplayer means the player is competing against at least one other person
* When two people are playing together, the two will play the game side-by-side on the screen, each person controlling one crab. 

### Nice-to-Have Features

* Multiplayer maps are the same randomly generated series of platforms 
* When multiple people are playing together, the players will all be rendered on the same map with the additional players having a lower opacity than the main player (like ghosts)
* More than two players allowed to play at the same time
* Different types of platforms such as ones that you can’t jump on or ones that you can only jump on once
* Different avatars have different physics associated with them
* Activate a rocket power-up to fly past platforms

## Why it’s interesting

We like the idea of developing a game in Rust to take advantage of its speed and safe-to-use development process. There are also several crates for game development already, so it will be interesting to explore what we can do in terms of creating a game.

Of course, there is also the systems aspect of the project, which is the networking/multiplayer functionality of the game. We hope to be able to have two players play against each other simultaneously with minimal latency. That will be an interesting challenge :)

## Anticipated Difficulties
* Keeping the multiplayer games in-sync
* Setting up a P2P connection that allows us to send game data with little latency
* Time crumch


## Examples


