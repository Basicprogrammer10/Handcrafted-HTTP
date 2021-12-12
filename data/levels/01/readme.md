### Challenge #01⏎
<br/>

When you need to *get* data from a server, you will use a **GET** request. In this first challenge, you will need to retrieve data from the path `/`<br><br/>

How do even structure a request in the first place?<br>Well, that follows this basic structure:

> <METHOD> <PATH> <VERSION><br><br><br><BODY>

- The **METHOD** is like the *operation* you want to execute
- The **PATH** is the *place* you are making this request to, think of a file path on your computer.
- The **VERSION** is just the version of HTTP the request is in. This will always be *HTTP/1.1* in our case.
- The **HEADERS** are for extra *metadata* with the request.
- The **BODY** is for sending data to the server. Don't worry about it... for now.

<br/>
Try to **GET** data from the **ROOT path** (*/*).

<br/>
If you are stuck, try again after reading [this](https://betterprogramming.pub/the-anatomy-of-an-http-request-728a469ecba9)⏎
