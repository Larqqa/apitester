# API Tester
## Simple cli tool to test some API endpoints

### How it works
You input the parameters you want to test and it writes the response to an output file. This is mainly designed for JSON based APIs but you can also define the result file as any type as long as it's representable by Reqwests text method.

### Type and URL
Basic use for the tool is to give a HTTP request type and url to do the given request to.
- Type is given with the `-t | --type` flag, and can be `GET | POST | PUT | PATCH | DELETE`
- URL is given with the `-u | --url` flag

### Resources folder
The tool uses `body.json` and `headers.toml` for resources you can give the requests. These are created by default if not found.
You can also define a custom path for the body with the `-b | --body` flag.

The body is handled either as a basic string that is given to `reqwest::Body` constructor, or as any valid JSON which is given to `reqwest::Json` constructor.

Headers should be defined using Toml formatting, for example a basic auth header should be given as
```toml
[[headers]]
name = "Authorization"
value = "bearer token"
```

### Output
Default output is in `output/result`.
- Destination folder path can be given with the `-p | --path` flag.
- Output file name can be given with the `-n | --name` flag.
- The file extension can be given with the `-e | --extension` flag.

### Development
There is a simple mock API using `mocks-server` that can be used to easily test the requests.
- Mock API starts by running `npm start` in the `testing_server` folder
- The server runs at `http://localhost:3100`