## General Philosophy
This project is born of the fact that I struggle to remember the most basic shit<br>
On far too many accasion I have just forzen up when I forget the irl name of an online friend iv known for 5+ years<br>
Or I feel like shit after forgeting its family member's birthday and i didnt even buy flowers<br>

The goal of this project is to act as a secondary memory/ formated notes system<br>
Instead of having to have pages of birthdays and middle names and favorite colors<br>
All the data can be written down in a single place and just as quickly recollected<br>

The most importantly this data it all stored securly and localy<br>
Cause given how this data is indexed, corps would be creaming their pants to get their hands on this shit<br>

The general structure of the project is a single user client/server<br>
With rust being used due to its speed (and me wanting to learn how tf to use it)<br>

## *Possible Future Plans*
None of this shit is fleshed out but there are a few things I would like to look into
- LLM data extraction (Look I dont like llms but given this project is supposed to be running localy and I have been in a situaiton where Ill remember something like 'Oh ye, Im busy on x but not on y" but not being able to conventinal search it, LLMs might actually be a valid solution)
- Plugins system (I want there to be someway to basicly create your own "APIs" so the official one could have maximum love but if someone needs something niche it doesnt harm the ecosystem)


## Current Ideas on Naming Things

Connection - The Person ur writing about<br>
Detail - The specific thing ur writing (Name, Favorite color, etc)<br>
Card - A simplfied visual way to find connections which contains some details<br>
Profile - The expanded view showing all details of a connection<br>
Template - How you custimize what shows up on a card/profile and how you add custom global details or types for details and interactions <br>

## Folder Structure WIP

probably going to be store in ~/.config/ bc like it just work <br>

```
tia-server/ 
├── index.db (primary db for storing the metadata about each connection/ might also have card details stored here)
├── templates.db (stores all the templates  
├── config.toml (for storing preferences and the like)
├── connections/
│   └── connection folders are named by connection id (uuid)/
│       ├── details.db
│       ├── interactions.db
│       └── raw-interactions/
│           └── 2026.01/ - folders are name after year.month the interaction was imported 
├── connection-pfps/
│   └── uuid of the connection.something (havent decieded yet probably just going to ffmpeg it into ones standard in the end)
└── backups/
    └── 1jan25a.iadv (named after day month year (letter noting which order backups where made that day a to z) which the backup was made. Its not really ment to be automaticly loaded so naming is human ment to be as human readable as possible. also its just a zip file of all the other files)
```
made using https://tree.nathanfriend.com/ <br>

## Stuff I plan on using
*mostly just a place to put shit so i dont forget*<br>

https://github.com/tokio-rs/axum - for the rest api<br>

https://github.com/diesel-rs/diesel - orm for the dbs<br>

## Api Endpoints

simple enpoints <br>
```    
api/v0/connections/
    post - create and returns a connection ID 
    get - returns all IDs of active connections

api/v0/connections/{id}
    get - returns metadata about a connection (ie. if it has a pfp set, its current status, etc)
    put - used for updating the status of a connection 
    delete - marks a connection for deletion

api/v0/connections/{id}/pfp
    put - setting the pfp of a connection

/api/v0/connections/{id}/details/
    post - creates a local detail 
    get - gets all details and their values from a connection

/api/v0/connections/{id}/details/{id}
    put - updates a details value

/api/v0/connections/{id}/interactions/
    post - creates a new interaction
    get - gets all interactions

/api/v0/connections/{id}/interactions/{id}
    put - updates and interaction

/api/v0/connections/{id}/interactions/{id}/raw/
    get - returns the file the interaction was imported from
```


