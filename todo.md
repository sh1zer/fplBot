Tracking fixtures,
what we need to do is 
- track channel info (channelid, tracked fixtures etc)
- periodically go through list of channels with active tracked fixtures, and send updates (nevermind, actually should be find player updates, update all channels that player/team is tracked by)

how do we get the updates?
- store current version of the /event/id/live data in the db, refetch and mark modifications

now should we do modifications -> channels or channels -> modifications?
so do we store information as user_tracking: [fixture_id, player_id, etc]
or player_being_tracked: [channel_id, user_id, etc]
2nd option seems far more reasonable as when quickly fetching the data we get to instantaneously update everything, without having to run through the entire dataset of everyone tracking everything


okay so we hold a table somewhere in the db, which now that i think about it doesnt even have array so the thing i said earlier really doesnt matter.
we just hold a bunch of [channel_id, fixture_id] pairs and then look that up however we want should also work with players etc since a couple hundred rows 


how do we mark modifications? previous assumption was we're just gonna reinsert the whole table but that wont work i dont think thats gonna work so i think the order will be:
- fetch data
- have pre-stored data on what players are actively playing ([TODO] we can somehow cache the player indexes using the db but still need to figure that out)
- check for changes in only that segment of the data (parsing json probably the biggest bottleneck here but should easily be fine performance-wise)
- send updates to some sort of handler that updates all the actively tracking channels. This does not need to very quick (up to 15/20 seconds delay i would say is acceptable), so we're not pressed for performance and i think we just move everything through the database for simplicity

handler itself should be pretty easy, just store a db table of [channel_id, fixture_id (can NULL), playher_id (can NULL)], and whenever you call the function with an array of modifications (probably in program memory, just a regular array of an enum with either ChannelMod or PlayerMod), send out all the updates, bada bing bada boom
need to figure out how to batch discord sends as im not fully familiar with the api and dont want to get rate limited


implementation order:
lets start with the db migrations for the data
gameweek data [implemented]

then a deserializer/db updater [WIP]
with a handler for detecting all modifications in general (probably in the same function honestly)

we can then implement the channel tracking things, such as the db migrations necessary and the update handler for discord itself, [TODO] figure this out later
