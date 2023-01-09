-- Your SQL goes here

PRAGMA foreign_keys=off;
--- drop the external source table entirely
DROP TABLE todos;
DROP TABLE events;
DROP TABLE userexperiencesettings;

CREATE TABLE externalsource(
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- unique id for external sources
    user_id INTEGER NOT NULL, -- which user is this external source configured for
    typ TEXT NOT NULL, -- what sort of external source is this?
    access_token TEXT, -- what, if any, token/password do we need to access this source
    user_name TEXT, -- what, if any, user name do we need to access this source
    source_uri TEXT NOT NULL -- where is this external source located

);

CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT, --- unique id for the todo
    user_id INTEGER NOT NULL, --- foreign key reference to the user who owns this setting row
    headline TEXT NOT NULL, --- the headline thing for this todo
    goal_time TEXT, --- when we would like to have this todo done by
    priority INTEGER, --- how important is this todo
    category_id INTEGER, --- the category that this belongs to
    external_source_id INTEGER, --- does this belong to us, or is this an external todo
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE, -- if a user is deleted, delete their events
    FOREIGN KEY(category_id) REFERENCES categories(id) ON DELETE SET NULL, -- if a category is deleted, reset the category
    FOREIGN KEY(external_source_id) REFERENCES externalsource(id) ON DELETE CASCADE -- if an external source is deleted, delete the source

);

CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT, --- unique id for events
    user_id INTEGER NOT NULL, --- foreign key reference to the user who owns this setting row
    headline TEXT NOT NULL, --- the headline name/description for this event
    start_time TEXT NOT NULL, --- when the event starts
    end_time TEXT NOT NULL, --- when the event ends
    description TEXT, --- what is the description for this event
    category_id INTEGER, --- what category does this event belong to
    external_source_id INTEGER, --- does this belong to us, or is it an external event
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE, -- if a user is deleted, delete their events
    FOREIGN KEY(category_id) REFERENCES categories(id) ON DELETE SET NULL, -- if a category is deleted, reset the category
    FOREIGN KEY(external_source_id) REFERENCES externalsource(id) ON DELETE CASCADE -- if an external source is deleted, delete the source

);

CREATE TABLE userexperiencesettings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL, --- foreign key reference to the user who owns this setting row
    time_zone TEXT NOT NULL, --- the time zone where this user operates
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);


PRAGMA foreign_keys=on;