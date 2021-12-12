### Challenge #04⏎
<br/>

The server gave the following response:
> HTTP/1.1 200 OK<br>Server: afire/0.2.2<br><br>Balance: -1.00

Looks like you have some debt! Anyway on to **Cookies**!

> HTTP cookies are small bits of data sent by a web server while a user is browsing a website and stored on the user's computer.

The web browser sends the cookies back to the server with every request.

<br/>
This useful for maintaining user sessions between pages. It is also used for tracking people across pages, collecting their data and using it for profit. (cough. cough. google).
Cookies are crucial to the web's infrastructure. You're now going to learn how they work! Whether you want to or not.

<br/>
After any **HTTP request**, the server's response may contain a `Set-Cookie` header. This gives the browser a Key, Value and some optional values.
Then after that, the browser will send all cookies of that site back with the `Cookie` header.

<br/>
The server has sent you the following cookies:

> double-chocolate = way-too-sweet

> oatmeal-raisin = mega-yummy-i-swear

> sugar-cookie = boring-but-blessed

Please make a **GET** request to `/cookie` and attach all cookies along with it.
