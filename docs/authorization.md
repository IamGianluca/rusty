## authorization

JSON Web Token (JWT) is a compact, self-contained means for securely transmitting information between parties as a JSON object. JWTs are often used to authenticate users and transmit claims or assertions about a user or other entities. They are commonly employed in web applications and APIs for ensuring the integrity and security of data transmission.

Here's a step-by-step explanation of how JWTs work:

1. Header: The JWT starts with a header typically consisting of two parts: the type of token (JWT) and the signing algorithm being used (e.g., HMAC SHA256 or RSA).

2. Payload: The second part of the token is the payload, which contains claims or assertions. Claims are statements about an entity, such as a user's name, roles, or permissions. The payload can also include custom claims.

3. Signature: To create the signature part you need to take the encoded header, the encoded payload, a secret, the algorithm specified in the header, and sign that.

The final JWT is a concatenation of these three parts, separated by periods. For example: `xxxxx.yyyyy.zzzzz`.

JWTs are used for various purposes, including:

- Authentication: Users can log in and receive a JWT, which they send with each subsequent request to prove their identity without having to send their credentials over and over.

- Authorization: The claims in a JWT can specify the user's role and permissions, allowing services to make authorization decisions.

- Information Exchange: JWTs are widely used for securely transmitting information between systems and services.

It's important to note that JWTs are typically signed to ensure their integrity, but they are not encrypted. They should not be used to transmit sensitive information that needs to be kept confidential.

For more information about JWT, please visit the [official page](https://jwt.io/introduction).
