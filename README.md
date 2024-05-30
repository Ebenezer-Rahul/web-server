# Web Server in rust

For now it can act as a web server which can serve basic html, css, js files (only text files i.e UTF-8 encoded files).

for now it kind of works like flask where you specify the route, method and write down a handler function.

this server leverages multiple threads to serve the users.

## TO-DO
The handler cannot pass the request json object yet.
Cannot serve image files/ Video files.

My hope for the project is to make it a static site generator files where you can write down a bunch of markdown and it will generate corresponding site.

## Note
This is just a project i built to learn rust. So the code is not neat/maintainable.
