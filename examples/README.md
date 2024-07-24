To run:

```
spin build
spin up --sqlite "CREATE TABLE pets (age INTEGER, name TEXT, is_finicky BOOL)" --sqlite "INSERT INTO pets(age, name, is_finicky) VALUES (18, 'Slats', true)" --sqlite "INSERT INTO pets(age, name, is_finicky) VALUES (1, 'Hobbes', false)"
```

(you should pass the `--sqlite` options only the first time you run or after zapping the database)

Note: The `Pet` model is the same across all examples. I was just lazy about extracting it to a common library.

# PostgreSQL

```
$ docker run --name my-postgres -e POSTGRES_PASSWORD=my_password -p 5432:5432 postgres
$ docker exec -it gallant_mcclintock bash  # substitute own container name for gallant_mcclintock
# in the docker terminal
$ createdb mydb -h localhost -U postgres
$ psql mydb -h localhost -U postgres
# in the psql shell
mydb=# CREATE TABLE pets (age int, name text, is_finicky boolean);
mydb=# INSERT INTO pets(age, name, is_finicky) VALUES (18, 'Slats', true);
# you should now be able to select * from pets;
mydb=# exit
# back in the docker terminal
$ exit
```
