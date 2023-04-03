## Features
This doc contains planned and completed features


### Developer productivity
> As a developer I want to be able to inspect the complete chat-gpt response in debug mode

> As a developer I want to quickly enable debug logging/capabilities quickly which enables me to see raw http responses and requests.

> As a developer I need to be able to keep track of the used tokens and the tokens that are left in a chat


### User features
> [DONE] As a user I want to remember the client to remember the chat history, so that I can utilize the model correctly

In the first iteration the history is only kept in memory and will be deleted after the program is closed.
We store the complete requests and responses in json. No need to just store partial things.


> As a user I want the history of a chat to be persisitent when closing asession so that I can continue a previous chat

> As a user I want to clean a chat session

> As a user I want to go back to old chat sessions

> As a user I want to be able to quickly distinguish between the sessions I had in the past

> As a user I want to quickly add files into a chat history so I can quickly get feedback on the contents of the file.

> As a user I want dont want to manually select files that should be added to the context, I want that a whole project is present as the context in the LLM, so I can utilize the full power of the model.

### Notes

Gerade in rust sah mein development prozess wie folgt aus: 
* baue feature
* compiler will nicht.
* bei unbekannten fehlern: knall den fehler in chat gpt
* copy paste den code von chatgpt
* compile
* nimm den fehler knall ihn in chatgpt
* compile
* rinse, repeat... 
--> Das kann man auch einfach automatisieren. Im besten Falle drueckt man einfach nur noch enter der rest wird generiert :D
