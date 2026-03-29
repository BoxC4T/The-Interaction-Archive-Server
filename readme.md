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
├── tia.db
├── config.toml
├── version.toml
├── raw-interactions/
│   └── 2026.01/
│       └── file.extention
├── connection-pfps/
│   └── uuid.png
└── backups/
    └── 1jan25f.iadv
```

tia.db - main db, should only contain processed data <br>
config.toml - server configs <br>
raw-interactions - storage for raw interactions, files are put into folder depending on the import date <br>
connection-pfps - pfp storage, file is named using the uuid of the connection <br>
backups - formated in day, month, year, amount of raw interactions contained (f - full, d - imported that day, w - imported that week, m - imported that month, y - imported that year, l - no interactions (lite), c - custom) <br>
extention comes from the Interaction Archive Data Vault and contains the main db, server config, interactions and pfps <br>
made using https://tree.nathanfriend.com/ <br>

## Stuff I plan on using
*mostly just a place to put shit so i dont forget*<br>

https://github.com/tokio-rs/axum - for the rest api<br>

https://github.com/diesel-rs/diesel - orm for the db<br>

## Api Endpoints

simple enpoints <br>
```    
/api/v0/connections/
    post - create and returns a connection ID 
    get - returns all IDs of active connections

/api/v0/connections/{id}
    get - returns metadata about a connection (ie. if it has a pfp set, its current status, etc)
    put - used for updating the status of a connection 
    delete - marks a connection for deletion

/api/v0/connections/{id}/pfp/
    get - returns the pfp of a connection
    put - setting the pfp of a connection

/api/v0/connections/{id}/details/
    post - creates a local detail 
    get - gets all details and their values from a connection

/api/v0/connections/{id}/details/{id}
    put - updates a details value

/api/v0/interactions/
    post - creates a new interaction
    get - gets all interactions

/api/v0/interactions/{id}
    put - updates and interaction

/api/v0/interactions/{id}/raw/
    get - returns the file the interaction was imported from
```


## Detail Types
```
  string - UTF8 encoded String
  stringArray - array of strings, has an optional max length field 
  formatedString - derived from string, has a format field for a regex expresion
  formatedAtringArray - derived from stringArray, has a format field which can either be a single expresion formating the whole array or an array of expresions formating the corresponding value
  
```
## Detail Structure
Details are stored as a json object with some default properties
```JSON
{
  "id":"bFirstName",
  "type":"string",
  "type_data":{},
  "confidance":100,
  "interactions":[
    "f044e2f2-d472-4296-946d-9d681cc6c461"
  ],
  "data":"Lukas"
}
```
id - first letter dictates source of datail (b - builtin, c - custom) <br>
for type and type_data see Detail Types <br>
confidance - number from 0 (pure guess) to 100 (known truth), defaults to -1 (unknown) <br>
interactions - array of interactions where the detail's information comes from <br>
data - the stored data of the detail, can be in multiple forms depending on its primitive <br>


## Builtin Detail 

| detail ID   | Implemented |
|-------------|-------------|
| bFirstName  | ([])        |
| bMiddleName | ([])        |
| bLastName   | ([])        |

## Interaction Structor

```JSON
{
  "id":"f044e2f2-d472-4296-946d-9d681cc6c461",
  "type":"message",
  "date_time":"2026-03-28T21:18:37Z",
  "interaction_source":{
    "file_dir":"2026.03/chat.zip",
    "file_metadata":{
      "type":"zip",
      "file_loc":"chats/day2.json",
      "line":"5"
    }
  },
  "summary":"",
  "raw_txt":"Hey im Lukas"
}
```
id - uuid of the interaction <br>
type - generic indicator for faster sorting though data <br>
date_time - single UTC or start, end obj
interaction_source - file_dir (where a file is located in the raw interactions folder) 
file_metadata - type (zip or file), zips have a file_loc while files dont, line (single or start, end obj) <br>
summary - for longer or non text interactions (here is where an llm might be nice) <br>
raw_txt - raw text pulled from the source file <br>

## Tables
connections - "entry point" table for storing connection IDS (uuid), connection status (active, archived, marked for deletion), if its marked what day it should be deleted, if the connection has a pfp, and probbly template stuff in the future <br>
interactions - where processed interactions are stored <br>
details - where details are stored <br>
