### Challenge #05⏎

> Redirects! luv em' or hate em' they are also a thing that website do sometimes. Not gunna lie they are not that fundamental but they still are important. Kind of. not really. But we'll also be teaching you about server responses!

When a server sends information to a client, the format is a bit different.

> <VERSION> <STATUS CODE> <REASON PHRASE><br>
> <HEADERS><br><br>
> <BODY>

- The **VERSION** is the same as before. Its `HTTP/1.1` in our case. Hasn't changed.
- The **STATUS CODE** is the status of the response. 200 is `OK`, 404 is `Not Found`, and 418, or `I'm a teapot`, etc.
- The **REASON PHRASE** is a short textual description of the STATUS CODE
- The **HEADERS** are the same as before.
- The **BODY** is also the same as before.

<br/>
Your challenge is as follows: Structure a server response, **redirecting** a user to another site (The response code in this case is **301** `Moved Permanently`). The page to redireect to is defined in the **Location** header.
