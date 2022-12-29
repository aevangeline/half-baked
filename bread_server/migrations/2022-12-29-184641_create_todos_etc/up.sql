-- Your SQL goes here
--- Creates the basic tables for ToDos, Events, and Categories

CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT, --- unique id for the todo
    user_id INTEGER NOT NULL, --- foreign key reference to the user who owns this setting row
    headline TEXT NOT NULL, --- the headline thing for this todo
    goal_time TEXT, --- when we would like to have this todo done by
    priority INTEGER, --- how important is this todo
    category_id INTEGER, --- the category that this belongs to
    external INTEGER, --- does this belong to us, or is this an external todo
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(category_id) REFERENCES categories(id)

);

CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT, --- unique id for events
    user_id INTEGER NOT NULL, --- foreign key reference to the user who owns this setting row
    headline TEXT NOT NULL, --- the headline name/description for this event
    start_time TEXT NOT NULL, --- when the event starts
    end_time TEXT NOT NULL, --- when the event ends
    description TEXT, --- what is the description for this event
    category_id INTEGER, --- what category does this event belong to
    external INTEGER, --- does this belong to us, or is it an external event
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(category_id) REFERENCES categories(id)

);

CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT, --- unique id for a category
    user_id INTEGER NOT NULL, --- foreign key reference to the user who owns this setting row
    name TEXT NOT NULL, --- the name for this category
    description TEXT, --- the description for category
    FOREIGN KEY(user_id) REFERENCES users(id)
);