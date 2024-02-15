To run:

```
spin build
spin up --sqlite "CREATE TABLE pets (age INTETGER, name TEXT, is_finicky BOOL)" --sqlite "INSERT INTO pets(age, name, is_finicky) VALUES (18, 'Slats', true)" --sqlite "INSERT INTO pets(age, name, is_finicky) VALUES (1, 'Hobbes', false)"
```

(you should pass the `--sqlite` options only the first time you run or after zapping the database)

Note: The `Pet` model is the same across all examples. I was just lazy about extracting it to a common library.
