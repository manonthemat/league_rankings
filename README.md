# league_rankings

The main purpose of this program is to read game data to create a top-3 ranking after every matchday.

While a greedy approach that solves this scenario might be best in the short-term, the way this program is structured allows us to further extend current capabilities with a flexible and easily comprehensible approach. This comes with a downside of allocating more memory than necessary to accomplish the initial task.

## Testing with local rust environment w/ cargo installed

```
cargo build --release

diff expected-output.txt <(./target/release/league_rankings sample-input.txt)

cargo test
```

## Docker

### Build

Build docker image from project directory:

```
docker build -t league_rankings .
```

### Test

```
docker run -it --rm league_rankings cargo test
```

### Run

The txt files are not part of the docker image -- as defined in the `.dockerignore` file.
To actually run the program, you need to add a volume when running the application in a docker environment.

Run this command from within a directory where you have (preferably only) your data available for processing:

```
docker run --rm -it -v $(pwd):/usr/src/league_rankings:ro league_rankings league_rankings /usr/src/league_rankings/sample-input.txt

```

This assumes the sample-input.txt to be present. Change the filename if needed.
