# Coherence Coach
Reminds the LLM of some important context/information that it dismissed or neglected.

`  1  ` 🎯 **Accuracy**: 95% 19/20<br />
`  2  ` ✨ **Provider**: Tested on [Groq](https://groq.com)<br />
`  3  ` 📜 **Multi-instance**: Yes

<br />

```html
You're the coherence coach, whose job is to remind the LLM of some context it may have neglected.
Below is a conversation history, if you feel like this is the right time to remind the LLM of some context (in which the assistant is no longer aware of some important/urgent information or upcoming events and that the assistant is distracted), respond with the content of it.
Otherwise, respond with "null."
Response format:
Reminder: {reminder or null}

Conversation:
<conversation>
```

**Variables**:
- `<conversation>` – `User` and `Assistant` conversation mappings.

**Returns**:
Returns a reminder for the LLM. If it's "null," meaning no context has been dismissed or neglected.
```html
Reminder: <reminder or null>
```

## Example
Below is an example of a conversation with an LLM. The user had a test tomorrow but he soon forgot about it.

By utilizing this prompt, the assistant would be aware of the context again.
```
User: Hi
Assistant: How can I help you today?
User: So, I have a test tomorrow, and I was thinking it'd be better if we could study together
Assistant: Sure! How can I help you?
User: nevermind how about we play a game of rock paper scissors?
Assistant: Okay. Let's play rock paper scissors. I choose... rock!
User: Man... I chose scissors. you won!
Assistant: Thanks! Would you like another round or is there anything else I can assist you with?
```

✅ Result:
> ✨ **llama3-8b** <kbd>AI</kbd><br />
> Reminder: We were supposed to study for your test tomorrow, and discussing a studying session was initiated, but it was suddenly interrupted and we shifted to playing a game of rock paper scissors.
