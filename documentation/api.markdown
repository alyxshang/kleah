# API Routes

## `/token/create`

This API route lets a user create a new API token. 
The names in the value fields represent the data types
expected of each key. Keys like `"can_change_pwd"` set the token's
permissions. 

- JSON payload:

```JSON
{
    "username": "string",
    "password": "string",
    "can_change_pwd": "bool",
    "can_set_mood": "bool",
    "can_delete_user": "bool",
    "can_change_email": "bool"
}
```

- JSON data returned:

```JSON
{
    "username": "string",
    "token": "string",
    "created_at": "string",
    "is_active": "bool",
    "can_change_pwd": "bool",
    "can_set_mood": "bool",
    "can_delete_user": "bool",
    "can_change_email": "bool"
}
```

## `/token/delete`

This API route lets a user delete an API token. 
The names in the value fields represent the data types
expected of each key. The `"number"` in the response object can be either one or zero. This is determined by whether the deletion was successful or not.

- JSON payload:

```JSON
{
    "username": "string",
    "password": "string",
    "api_token": "string"
}
```

- JSON data returned:

```JSON
{
    "status": "number"
}
```

## `/user/delete`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
```

- JSON data returned:

```JSON
```

## `/user/create`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
```

- JSON data returned:

```JSON
```

## `/mood/create`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
```

- JSON data returned:

```JSON
```

## `/mood/delete`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
```

- JSON data returned:

```JSON
```

## `/user/update/pwd`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
```

- JSON data returned:

```JSON
```

## `/user/update/email`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
```

- JSON data returned:

```JSON
```

## `/mood/get`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
{
    "username": "string"
}
```

- JSON data returned:

```JSON
{
    "username": "string",
    "is_active": "bool",
    "mood": "string",
    "created_at": "string"
}
```

## `/moods/get`

This API route lets a user create a new API token. 

- JSON payload:

```JSON
{
    "username": "string"
}
```

- JSON data returned:

```JSON
{
   "active_mood": "current active mood",
   "inactive_moods": ["moods"]
}
```

## `/tokens/get`

This API route lets a user create a new API token. 
The names in the value fields represent the data types
expected of each key.

- JSON payload:

```JSON
{
    "username": "string",
    "password": "string"
}
```

- JSON data returned:

```JSON
{
    "tokens": ["API tokens"]
}
```

## `/files/upload`
## `/email/verify/{email_token}`
