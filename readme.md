Crate contains implementation of simple spell checker proposed by Norvig. See https://norvig.com/spell-correct.html.

The idea woudl be to play with Rust and see how it can be implemented with the focus on:
1. ~~Handling files~~
2. Data manipulation code.
3. ~~Adding CLI interface.~~
4. Using crate in Python - Foreign Function Interfaces (FFI) and see how easy/hard it is.
5. Convert counter into generic
6. Microbenchmarking
7. Parallelization - TBB pipeline style.
8. ~~Multiple asserts in unit tests~~
9. ~~Reading form file - nice, Use a pipe there to read line and pass to counter~~
10. ~~Make it API.~~
11. ~~Add subcommand:~~
    a. ~~to pass corrections and output in cmd~~
    b. ~~to start server and process requests.~~
    
12. ~~Build docker image~~
13. Initalize spell check at the start and pass it to service handler function.
14. Add to readme how program can be used in 
15. Download corpus file on fly at startup.
16. Add k8s deployment.
17. Comments in Rust - how?
18. Add readme

```commandline
docker build --tag spell-checker-api-docker-image .
docker run --init -p:1234:8315 spell-checker-api-docker-image
```

The last command is exposing service locally on `0.0.0.0:1234` (mapping from `80`).

## Command line interface

To use a command line interface to Norvig Spell Checker, navigate to a directory where executable is located. 

Execute:

```commandline
./norvig_spell_checker correct speling korrectud bycycle inconvient arrainged peotry peotryy word quintessential
```

for the following words to be corrected: speling korrectud bycycle inconvient arrainged peotry peotryy word quintessential.

The output should look similar to:

```
[2021-01-31T20:21:43Z INFO  norvig_spell_checker] Running Norvig Spell Checker built on 31-JAN-2021
[2021-01-31T20:21:43Z INFO  norvig_spell_checker] Using default corpus file
[2021-01-31T20:21:43Z INFO  norvig_spell_checker::spell_checker] Using corpus file located at /home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt
[2021-01-31T20:21:43Z INFO  norvig_spell_checker::spell_checker] Using 4 threads
[2021-01-31T20:21:43Z INFO  norvig_spell_checker::spell_checker] Spell checker initialized. It took 172.870155ms to instantiate (par). 75138 words processed
[2021-01-31T20:21:43Z INFO  norvig_spell_checker::command_line_corrections] Words to correct: ["speling", "korrectud", "bycycle", "inconvient", "arrainged", "peotry", "peotryy", "word", "quintessential"]
[2021-01-31T20:21:43Z INFO  norvig_spell_checker::command_line_corrections] It took 143.757638ms to find corrections for words
┌────────────────┬────────────────┐
│ Word           │ Correction     │
├────────────────┼────────────────┤
│ speling        │ spelling       │
│ korrectud      │ corrected      │
│ bycycle        │ bicycle        │
│ inconvient     │ convient       │
│ arrainged      │ arranged       │
│ peotry         │ poetry         │
│ peotryy        │ poetry         │
│ word           │ word           │
│ quintessential │ quintessential │
└────────────────┴────────────────┘
```

## REST API - k8s deployment and usage

In order to deploy the `spell-checker-api` on k8s using minikube follow the steps:

1. Start minikube and configure minikube to use local docker registry.

```commandline
minikube start --cpus 6
eval $(minikube docker-env)
```

2. Build a docker image containing `spell-checker-api`.

```commandline
docker build -t spell-checker-api-docker-image .
```

3. Deploy a service and start it:

```commandline
kubectl delete deployment spell-checker-api
kubectl apply -f k8s-deploy.yaml
minikube service spell-checker-api-service
```

After executing the second command, you should see something similar to:

```commandline
|-----------|-----------------------|-------------|---------------------------|
| NAMESPACE |         NAME          | TARGET PORT |            URL            |
|-----------|-----------------------|-------------|---------------------------|
| default   | spell-checker-service |        6005 | http://192.168.49.2:30102 |
|-----------|-----------------------|-------------|---------------------------|
🎉  Opening service default/spell-checker-service in default browser...
```

At this point you should be able to connect to your service - let's try to do it, use an URL from the last column:

```commandline
curl --request GET \
  --url http://192.168.49.2:30102/correction \
  --header 'Content-Type: application/json' \
  --data '{
	"words": [
		"speling",
		"korrectud",
		"bycycle",
		"inconvient",
		"arrainged",
		"peotry",
		"peotryy",
		"word",
		"quintessential"
	]
}'
```

A response should be:

```json
{
  "corrections": [
    {
      "word": "speling",
      "correction": "spelling"
    },
    {
      "word": "korrectud",
      "correction": "corrected"
    },
    {
      "word": "bycycle",
      "correction": "bicycle"
    },
    {
      "word": "inconvient",
      "correction": "convient"
    },
    {
      "word": "arrainged",
      "correction": "arranged"
    },
    {
      "word": "peotry",
      "correction": "poetry"
    },
    {
      "word": "peotryy",
      "correction": "poetry"
    },
    {
      "word": "word",
      "correction": "word"
    },
    {
      "word": "quintessential",
      "correction": "quintessential"
    }
  ]
}
```
