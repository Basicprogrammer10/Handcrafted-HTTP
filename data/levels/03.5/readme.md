### Challenge #03.5⏎

<br/>
The server gave the following response:
> HTTP/1.1 200 OK<br>Server: afire/0.2.2<br><br>K28x4spWnd

As you can see your request went through successfully and you got your token! (**K28x4spWnd**)

<br/>
Now you can send the *authentication token* with a **GET** request to `/api/balance`, and then *FINALLY* see how poor you are!

<br/>
To do this, we will use a **Header**.

> Headers let the client and the server pass additional information with an HTTP request or response

They have a _Key_ and a _Value_ separated by a `:`. Pass the token in a header with the name `Token`;
