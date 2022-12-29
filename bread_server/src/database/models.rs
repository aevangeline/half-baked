/// models.rs - this is where the basic datamodels used by half-baked
/// for half-baked, we've decided to use a pretty simple set of diesel models that are queryable 
/// these models should NOT be complex since this app is focused on simplicity & extensibility

// load up the diesel model with all the goodies that we need
use diesel::prelude::*;

/// User - a user is a specific person who has a local instance of half-baked running
/// have some basic information about them that we can use to configure their experience
/// configuration values in the user should be specific to the _user as a person_
#[derive(Queryable)]
pub struct User {
    pub id: i64, // the unique user-id for a given application user, never re-used
    pub given_name: String, // the given name for the user, has a default but we do need _something_
    pub last_name: Option<String>, // the last name for the user, not necessary for the application and not everyone has one!
    pub honorific: Option<String>, // the preferred honorific for the user, not necessary as well
}

/// UserExperienceSettings - this a table that stores all of the settings associated with a given user for their use of the half-baked application!
/// this table exists 1:1 with the User table, though the settings row for a user can be lazily created
#[derive(Queryable)]
pub struct UserExperienceSettings {
    pub id: i64, // unique id for this user's batch of settings
    pub user_id: i64, // user_id that tells us whose settings these are
    pub time_zone: String, // this application manages all date-times in UTC but then displays everything in local time so we need to store tz
}