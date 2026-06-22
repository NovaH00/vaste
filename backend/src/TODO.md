Explanation for each features

# `workspaces/`
A workspace is the top-level container, it contains nodes, metadata, ...

# `nodes/`
A node represent an item in a workspace, a node can be the following:
- A folder: contains other nodes and metadata (like description about the folder, datetime,...)
- A text file: format is not chosen, maybe MD syntax like Obsidian?
- A Note board: A board where i can put up sticky notes
- A todo board: A board where i can put todos
- A timeline: A board where i can put the events (or reference from other nodes, but idk how to
do this, yet), then it render the timeline
- etc...

# `auth/`
Just basic auth, we dont need any fancy authenticate system like oauth or whatever!

# `users/`
Just do basic users management.

# `agents/`
AI agents that will automate some of the tasks in the app

# `chat/`
Chat interface that let me chat with the AI agents and asking about the workspaces/nodes

