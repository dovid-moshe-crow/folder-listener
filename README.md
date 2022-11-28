# folder-listener

## usage 
#### The program takes a `path` to the folder that should be listened to, and the `url` to send a get request with a `filename` param with the file name of the created file.
### you can send optional `params` which the program would send to the webhook as query params.
```bash
  folder-listener <folder path> <url> param1=... param2=... ...
```