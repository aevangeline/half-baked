-- Basic DB Layout 
--- users and userexperiencesettings tables

--- users - stores all the basic information about a user 
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- unique id for a given user
    given_name TEXT NOT NULL, -- the user's given name
    last_name TEXT, -- the user's last name
    honorific TEXT -- the user's title (Mx., Mys., etc.)
) STRICT;

--- userexperiencesettings - stores all of the settings associated with a given user for the user interface
create TABLE userexperiencesettings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    time_zone TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
) STRICT;