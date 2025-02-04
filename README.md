
You can use Postman to test your Actix-Web API by sending HTTP requests to http://127.0.0.1:8080. Follow these steps:

1️⃣ Create User
Method: POST
URL: http://127.0.0.1:8080/users
Headers:

Content-Type: application/json
Body (JSON):

json
Copy
Edit
{
  "fullname": "John Doe"
}
Response:

200 OK → Returns a generated user ID (user_xxx).
2️⃣ Get User
Method: GET
URL: http://127.0.0.1:8080/users/{pid}
(Replace {pid} with the actual user ID from the previous response.)

Example:
http://127.0.0.1:8080/users/user_123456

Response:

200 OK → Returns user data:
json
Copy
Edit
{
  "fullname": "John Doe"
}
404 Not Found → If the user does not exist.
3️⃣ Update User
Method: PUT
URL: http://127.0.0.1:8080/users/{pid}
(Replace {pid} with the actual user ID.)
Headers:

Content-Type: application/json
Body (JSON):

json
Copy
Edit
{
  "fullname": "Jane Doe"
}
Response:

200 OK → "User updated"
500 Internal Server Error → If something goes wrong.
4️⃣ Delete User
Method: DELETE
URL: http://127.0.0.1:8080/users/{pid}
(Replace {pid} with the actual user ID.)

Response:

200 OK → "User deleted"
500 Internal Server Error → If something goes wrong.
