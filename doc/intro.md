# Game system overview

Mutemaanpa-crpg is a computer role playing game. Its features:

## Role play, interactive and story telling (purpose)

The most important thing in CRPG is to give a purpose. Why play this game?

DM can tell the player what to do, but better yet, they give player the story, and player fullfil
a role. Then, they can figure out what to do together.

In a video game, quest and dialogue give player these information.

To implement a quest system, we need:

- a data structure(DAG) to store all quest nodes
- a UI to display quests information
- a data structure binding to user, telling which quests are active on their side.
    an event mechanism to update quest status
    after quest finished, give user some incentive.

To implement a dialogue system, we need:

- a data structure(DAG still) to store all dialogue nodes
- a UI to display dialogue
- a FSM attached to everyone, storing what they are saying, and calculate what they can say, and what they
next should say.
- a state attached to each character, telling they are in talk status or not talking status.

## Agent, DM and the world (exploration)

## dungeon, enemy and battle (problem solving)
