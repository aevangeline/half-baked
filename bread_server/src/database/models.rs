use crate::database::schema::categories;
use chrono::NaiveDateTime;
/// models.rs - this is where the basic datamodels used by half-baked
/// for half-baked, we've decided to use a pretty simple set of diesel models that are queryable
/// these models should NOT be complex since this app is focused on simplicity & extensibility
// load up the diesel model with all the goodies that we need
use diesel::prelude::*;

/// User - a user is a specific person who has a local instance of half-baked running
/// have some basic information about them that we can use to configure their experience
/// configuration values in the user should be specific to the _user as a person_
#[derive(Queryable, PartialEq, Clone)]
pub struct User {
    pub id: i64,            // the unique user-id for a given application user, never re-used
    pub given_name: String, // the given name for the user, has a default but we do need _something_
    pub last_name: Option<String>, // the last name for the user, not necessary for the application and not everyone has one!
    pub honorific: Option<String>, // the preferred honorific for the user, not necessary as well
}

/// UserExperienceSettings - this a table that stores all of the settings associated with a given user for their use of the half-baked application!
/// this table exists 1:1 with the User table, though the settings row for a user can be lazily created
#[derive(Queryable, PartialEq, Clone)]
pub struct UserExperienceSettings {
    pub id: i64,           // unique id for this user's batch of settings
    pub user_id: i64,      // user_id that tells us whose settings these are
    pub time_zone: String, // this application manages all date-times in UTC but then displays everything in local time so we need to store tz
}

/// ToDo -- these represent ToDo that a user wants to complete, maybe with some sort of target completion date and a priority
/// for now, there is a 1:Many relationship between a user and todos, as such you can't have floating todos with no assignee or multiple assignees
#[derive(Queryable, PartialEq, Clone)]
pub struct ToDo {
    pub id: i64,                          // unique id for this todo item
    pub user_id: i64,                     // user_id that tells us whose todo this is
    pub headline: String, // the headline for this todo, what are we actually doing with this todo
    pub goal_time: Option<NaiveDateTime>, // when we want to get this todo item done by
    pub priority: Option<i64>, // how important is this todo? lower is more important
    pub category_id: Option<i64>, // what (if any) category is this todo filed under
    pub external_source_id: Option<i64>, // did this ToDo come from half-baked or is it imported from another program
}

/// Event -- these represent calendar events, with a begin and end time.
/// there is a 1:Many relationship between a user and an event, so each event is something that belongs to a specific user
#[derive(Queryable, PartialEq, Clone)]
pub struct Event {
    pub id: i64,                     //unique id for this event
    pub user_id: i64,                // user_id that tells us whose event this is
    pub headline: String,            // what the name of this event
    pub start_time: NaiveDateTime,   // when the event actually starts
    pub end_time: NaiveDateTime,     // when the event ends
    pub description: Option<String>, // what is the description of this event
    pub category_id: Option<i64>,    // what (if any) category is this event filed under
    pub external_source_id: Option<i64>, // did this event come from half-baked, or is it externally sourced
}

/// Category -- these are categories that you can optionally use to label events and todos
/// 1:Many with users table, each user has their own categories for their calendar
#[derive(Queryable, PartialEq, Clone)]
#[diesel(table_name = categories )]
pub struct Category {
    pub id: i64,                     // unique id for this event
    pub user_id: i64,                // user_id tells us whose category this is
    pub name: String,                // what is the name of this category
    pub description: Option<String>, // a description for this category
}

/// ExternalSource - this is a table that lists where we source external events, todos, and other data bits
#[derive(Queryable, PartialEq, Clone)]
pub struct ExternalSource {
    pub id: i64, // unique id for this external data source
    pub user_id: i64, // unique user_id who owns this source
    pub typ: String, // what type of source is this?
    pub access_token: Option<String>, // unique access token used to acess this source
    pub user_name: Option<String>, // the user name (if needed) to access this resource
    pub source_uri: String, // where is this external source located?
}
