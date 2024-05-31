## Features
This doc contains planned and completed features


### Developer productivity
> [DONE] As a developer I want to be able to inspect the complete chat-gpt response in debug mode

Debug mode is enabled by the rust env_logger. Improvement would be sl4rs, but not needed yet. To run in debug mode see the make file. 

> [DONE ]As a developer I want to quickly enable debug logging/capabilities quickly which enables me to see raw http responses and requests.

Debug mode is enabled by rust logging framework and env variable 'RUST_LOG'. Added convenience method to makefile to quickly debug chat bot.

> As a developer I need to be able to keep track of the used tokens and the tokens that are left in a chat


### User features
> [DONE] As a user I want to remember the client to remember the chat history, so that I can utilize the model correctly

In the first iteration the history is only kept in memory and will be deleted after the program is closed.
We store the complete requests and responses in json. No need to just store partial things.


> As a user I want the history of a chat to be persisitent when closing asession so that I can continue a previous chat

> As a user I want to clean a chat session

> As a user I want to go back to old chat sessions

> As a user I want to be able to quickly distinguish between the sessions I had in the past

> [DONE] As a user I want to be able to use the contents of my clip board to be properly inserted into the chat window. Right now a line break in my clip board content will be interpreted as an enter terminating signal for the user input

The investigation for this topic can be found at [docs/notes_clip_board_content.md](docs/notes_clip_board_content.md)

> [DONE] As a user I want to be able to add a hardcoded code file as the context for my conversation

> [DONE] As a user I want to determine the location of the file that is set as the context of my conversation.

> [DONE] As a user I want to select the file/text content that is set as the context of my conversation, so that I can use the client for any use case.

> [DONE] As a user I want to select multiple files as the context of my conversation, so that I can provide a broader context for my conversation.

> As a user I want dont want to manually select files that should be added to the context, I want that a whole project is present as the context in the LLM, so I can utilize the full power of the model.

> As a user I want to my git commits / file changes to be of high quality. Feature Idea: Leverage LLMs for this: THe LLM would run on each git diff or git commi and enhance the code 

> As a user I want a session not to crash because the max token limit is exceeded

## Tech & Architecture related

> Look into openAI embeddings for the file context. Apparently embedding are a cheaper way of getting 'context' to a chat. The text (files) that the user wants to work with would be somehow tokenized and pre trained and embedded into the openai model, I don't really undestsand yet how this works. The benefit is that this is chaper, consumes no token limit and ..

The above assumption is wrong, embeddings are a different concept.

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
