### Challenge #04⏎

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

Please make a **GET** request to `/cookie` and attach all cookies along with it (make sure they're in the right order we're too lazy to check).
