### Challenge #01⏎
<br/>

When you need to _get_ data from a server you will use a **GET** request. In this first challenge you will need to retrieve data from the path `/`<br><br/>

How do even structure a request in the first place?<br>Well that follows this basic structure:

> `<METHOD> <PATH> <VERSION><BODY>`

- The **METHOD** is like the _operation_ you want to execute
- The **PATH** is the _place_ you are making this request to, think of a file path on your computer.
- The **VERSION** is just the version of HTTP the request is in. This will always be _HTTP/1.1_ in our case.
- The **BODY** is for sending data to the server. Don't worry about it... for now.

<br/>
If you are stuck try again after reading [this](https://betterprogramming.pub/the-anatomy-of-an-http-request-728a469ecba9)⏎

### Challenge #02⏎

<br/>

> A **POST** request, in simple terms, is a way for you to _send_ data to a destination with the help of the internet.

When you need to **send** data to a server you will use a **POST** request.

<br/>
In this challenge you will need to send the data `Hello World` to the path `/api`.
Use what you learned in the first challenge!

<br/>
Make sure your data is in the correct format, or the server may not accept it.

> Hint: you need to worry about that `<BODY>` now.

The `<BODY>` section of a HTTP request also is a little more complicated.
It's format is:
>`<HEADERS><br><br><DATA>`

- The **\<br>**'s are there for formatting. They're HTML line breaks, if you're curious, just know that you need them in there.
- The **HEADERS** is where you put headers. They go one line after the main command of the request. This is additional information you may want to attach along with the request. This may take up multiple lines.
- The **DATA** is where you throw data. It's different from headers I swear! They go 2 lines after the headers (if there are any). This is where you'll want to attach `Hello World`

> remember `<br>`
### Challenge #03⏎
<!--
GET
/auth
HTTP/1.1
<br>Username:Nelly
<br>Password:password123!

POST
/bank
HTTP/1.1
Authentication-Token:token
-->
> Things get a little _wackier_ from here

You now want to see _how poor_ you are.
You want to use _[BANK NAME]s_ **COBOL API** to check, but it requires an _authentication token_.

<br/>
You can get said *authentication token* from `/auth` with a **POST** request.

<br/>
You will need to supply the username and password in the **BODY**.
For this challenge we will use the following Login Details.
- **Username:** Nelly
- **Password:** password123!

<br/>
Now you can send the *authentication token* with a **GET** request to `/api/balance`, and then *FINALLY* see how poor you are!

<br/>
To do this we will use a **Header**.

> Headers let the client and the server pass additional information with an HTTP request or response

They have a _Key_ and a _Value_ separated by a `:`, you don't know the value yet, just use `token` as a stand in. For the key we're using _Authentication-Token:_.

### Challenge #04⏎
<!--
GET
/cookie
HTTP/1.1
<br>Cookie:
double-chocolate=way-too-sweet;
oatmeal-raisin=mega-yummy-i-swear;
sugar-cookie=boring-but-blessed;
-->

<br/>
> HTTP cookies are small bits of data sent by a web server while a user is browsing a website and stored on the user's computer.

The web browser sends the cookies back to the server with every request.

<br/>
This useful for maintaining user sessions between pages. It is sometimes used for tracking people across pages, collecting their data and using it for profit. (cough. cough. google).

<br/>
Cookies are crucial to the webs infrastructure. And you're going to learn how they work! Whether you want to or not.

<br/>
After any **HTTP request**, the server's response may contain a `Set-Cookie` header. This gives the browser a Key, Value and some optional values.

Then after that whenever a request is made from the browser to the server, all cookies will also be supplied with the `Cookie` header.

<br/>
Your challenge is this: the server has send you the following cookies:

> double-chocolate = way-too-sweet

> oatmeal-raisin = mega-yummy-i-swear

> sugar-cookie = boring-but-blessed

Please make a **GET** request to `/cookie` and attach all cookies along with it.

### Challenge #05⏎
<!--
HTTP/1.1
301
Moved Permanently
Location:
/cookie_new

GET
/cookie_new
HTTP/1.1
-->

> Redirects! luv em' or hate em' they are also a thing that website do sometimes. Not gunna lie they are not that fundamental but they still are important. Kind of. not really. But we'll also be teaching you about server responses!

<br/>
When a server sends information to a client, it's format is a bit different.

>`<VERSION> <RESPONSE CODE> <BODY>`

- The **VERSION** is the same as before. its `HTTP/1.1` in our case. Hasn't changed.
- The **RESPONSE CODE** is the status of the response. 200 is `OK`, 404 is `Not Found`, and 418, or `I'm a teapot`, is what a server responds with to indicate it refuses to brew coffee because it is, permanently, a teapot. A combined coffee/tea pot that is temporarily out of coffee should instead return 503 (yes this is real).
- The **BODY** is in the same format as requests. Header, data, the usual.

Your challenge is this: structure a server response, redirecting a user to another site (The response code in this case is 301 `Moved Permanently`). Then you make a simple **GET** request from the client and access this new address.

### Challenge #06⏎
starting response:
```http
HTTP/1.1 200 OK
Set-Cookie:yummy-cookie=for-grandma

Please POST this cookie to '/grandma'
```

expected reply:
```http
POST /grandma HTTP/1.1
Cookie:yummy-cookie="for-grandma"
```

response:
```http
HTTP/1.1 300 Multiple Choice
Location:/not-grandma; /grandma-new; /also-not-grandma;
```

expected reply:
```http
POST /grandma-new HTTP/1.1
Cookie:yummy-cookie=for-grandma
```

response:
```http
HTTP/1.1 402 Payment required

Unfortunately you haven't supplied us with your account password! please GET it from '/bank' and give it to us along with the original cookie to continue!
```

expected reply:
```http
GET /bank HTTP/1.1
```

response:
```http
HTTP/1.1 200 OK

password123
please put this along with the cookie, in the header 'Password:'
```

expected reply:
```http
POST /grandma-new HTTP/1.1
Cookie:yummy-cookie=for-grandma
Password:password123
```

response:
```http
HTTP/1.1 418 I'm a teapot
Grandma:is happy

Congrats on making it through! Hopefully you now know how to make HTTP requests, and yes this information is totally useful in your every day life :tea:
```
