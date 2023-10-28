## authorization

JSON Web Token (JWT) is a compact, self-contained means for securely transmitting information between parties as a JSON object. JWTs are often used to authenticate users and transmit claims or assertions about a user or other entities. They are commonly employed in web applications and APIs for ensuring the integrity and security of data transmission.

The JWT contains three parts:

1. Header: The JWT starts with a header typically consisting of two parts: the type of token (JWT) and the signing algorithm being used (e.g., HMAC SHA256 or RSA).

2. Payload: The second part of the token is the payload, which contains claims or assertions. Claims are statements about an entity, such as a user's name, roles, or permissions. The payload can also include custom claims.

3. Signature: To create the signature part you need to take the encoded header, the encoded payload, a secret, the algorithm specified in the header, and sign that.

The final JWT is a concatenation of these three parts, separated by periods. For example: `xxxxx.yyyyy.zzzzz`.

For more information about JWT, please visit the [official page](https://jwt.io/introduction).


#### Authentication Workflow

When a client authenticates with a REST API using JSON Web Tokens (JWT), the process typically involves multiple requests. Below are the steps typically involved in this authentication process:

1. Initial Authentication Request:
   - The client initiates the authentication process by sending a POST request to the API's authentication endpoint.
   - The content of this request typically includes the client's credentials, such as a username and password.
   - The goal of this request is to exchange the user's credentials for a JWT.

2. Authentication Processing:
   - The API server receives the initial authentication request and validates the client's credentials.
   - If the credentials are valid, the server generates a JWT that contains claims about the user (e.g., user ID, roles, and expiration time).
   - The server signs this JWT with a secret key known only to the server. The resulting JWT is then returned in the response to the client.

3. JWT Response:
   - The response to the initial authentication request contains the JWT in the response body or a specific header (e.g., the "Authorization" header).
   - The client receives the JWT and stores it securely. It should typically be stored in a secure manner, such as in a client-side storage like cookies, local storage, or a mobile app's keychain.

4. Subsequent API Requests:
   - For all subsequent requests to protected resources on the API, the client includes the JWT in the request headers.
   - The client attaches the JWT as a bearer token in the "Authorization" header, typically in the format "Bearer <JWT>".

5. API Authentication and Authorization:
   - The API server, upon receiving requests with JWTs, verifies the JWT's signature using the secret key.
   - It also checks the JWT's expiration time and ensures it's not tampered with.
   - Once the JWT is successfully validated, the server extracts the user claims from the token.
   - The server then uses these claims to authenticate and authorize the client for the requested resource.

6. Response to Subsequent Requests:
   - If the JWT is valid and the client is authorized, the API processes the request and sends a response to the client.

In summary, the client initially sends one request to authenticate and receive a JWT. Subsequently, for each protected resource request, the client includes the JWT in the request headers. The goal is to provide secure and efficient authentication and authorization for the client while protecting the API's resources. This approach avoids the need for the client to repeatedly send sensitive credentials with each request, enhancing security and user experience.
