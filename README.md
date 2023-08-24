# NoobHandler

A Simple CLI that I wrote to handle my mongoDB database that is linked to various ascepts of my site [noobscience.rocks](https://noobscience.rocks)

You can't really use it since you need my unique `MONGODB_URL`, but you can use it as a template for your own CLI.

I just wanted to make a CLI that I could use to handle my database and since I wanted a nice little executable that I could run from anywhere, I decided to make it in rust.

## You can use this

I mean, if you want to, you can use this, but you'll have to change the `MONGODB_URL` to your own database. 
Your MongoDB database will need to have the following collections:
- "page"
- "code"
- "go"

The page collection will need to be of this format:
```json
{
    "_id": "ObjectId",
    "hash": "String",
    "name": "String",
    "content": "String",
    "date": "String",
    "author": "String",
}
```

The code collection will need to be of this format:
```json
{
    "_id": "ObjectId",
    "hash": "String",
    "title": "String",
    "content": "String",
    "lang": "String",
    "author": "String",
}
```

The go collection will need to be of this format:
```json
{
    "_id": "ObjectId",
    "url": "String",
    "slug": "String",
}
```

Then, hopefully, you should be able to run `cargo build --release` and then move the executable to your path.

I mean, if you want to, the code is quite easily understandable, so you should be able to make a new collection and then change the code to fit your needs.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details